"""Helper functions for converting streams to inline html."""

from __future__ import annotations

import logging
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from .blueprint import BlueprintLike

from rerun import bindings

from .recording_stream import RecordingStream, get_data_recording

_default_width = 640
_default_height = 480


def set_default_size(*, width: int | None, height: int | None) -> None:
    """
    Set the default size for the viewer.

    This will be used for any viewers created after this call.

    Parameters
    ----------
    width : int
        The width of the viewer in pixels.
    height : int
        The height of the viewer in pixels.

    """

    global _default_width, _default_height
    if width is not None:
        _default_width = width
    if height is not None:
        _default_height = height


_version_mismatch_checked = False


class Viewer:
    """
    A viewer embeddable in a notebook.

    This viewer is a wrapper around the [`rerun_notebook.Viewer`][] widget.
    """

    def __init__(
        self,
        *,
        width: int | None = None,
        height: int | None = None,
        blueprint: BlueprintLike | None = None,
        recording: RecordingStream | None = None,
    ):
        """
        Create a new Rerun viewer widget for use in a notebook.

        Any data logged to the recording after initialization will be sent directly to the viewer.

        This widget can be displayed by returning it at the end of your cells execution, or immediately
        by calling [`Viewer.display`][].

        Parameters
        ----------
        width : int
            The width of the viewer in pixels.
        height : int
            The height of the viewer in pixels.
        recording:
            Specifies the [`rerun.RecordingStream`][] to use.
            If left unspecified, defaults to the current active data recording, if there is one.
            See also: [`rerun.init`][], [`rerun.set_global_data_recording`][].
        blueprint:
            A blueprint object to send to the viewer.
            It will be made active and set as the default blueprint in the recording.

            Setting this is equivalent to calling [`rerun.send_blueprint`][] before initializing the viewer.

        """

        try:
            global _version_mismatch_checked
            if not _version_mismatch_checked:
                import importlib.metadata
                import warnings

                rerun_notebook_version = importlib.metadata.version("rerun-notebook")
                rerun_version = importlib.metadata.version("rerun-sdk")
                if rerun_version != rerun_notebook_version:
                    warnings.warn(
                        f"rerun-notebook version mismatch: rerun-sdk {rerun_version}, rerun-notebook {rerun_notebook_version}",
                        category=ImportWarning,
                    )
                _version_mismatch_checked = True

            from rerun_notebook import Viewer as _Viewer  # type: ignore[attr-defined]
        except ImportError:
            logging.error("Could not import rerun_notebook. Please install `rerun-notebook`.")
            hack: Any = None
            return hack  # type: ignore[no-any-return]

        recording = get_data_recording(recording)
        if recording is None:
            raise ValueError("No recording specified and no active recording found")

        self._recording = recording

        self._viewer = _Viewer(
            width=width if width is not None else _default_width,
            height=height if height is not None else _default_height,
        )

        bindings.set_callback_sink(
            recording=RecordingStream.to_native(self._recording),
            callback=self._flush_hook,
        )

        if blueprint is not None:
            self._recording.send_blueprint(blueprint)  # type: ignore[attr-defined]

    def display(self, block_until_ready: bool = True) -> None:
        """
        Display the viewer in the notebook cell immediately.

        Parameters
        ----------
        block_until_ready : bool
            Whether to block until the viewer is ready to receive data. If this is `False`, the viewer
            will still be displayed, but logged data will likely be queued until the viewer becomes ready
            at the end of cell execution.

        """

        from IPython.display import display

        display(self._viewer)

        if block_until_ready:
            self._viewer.block_until_ready()

    def _flush_hook(self, data: bytes) -> None:
        self._viewer.send_rrd(data)

    def _repr_mimebundle_(self, **kwargs: dict) -> tuple[dict, dict] | None:  # type: ignore[type-arg]
        return self._viewer._repr_mimebundle_(**kwargs)  # type: ignore[no-any-return]

    def _repr_keys(self):  # type: ignore[no-untyped-def]
        return self._viewer._repr_keys()

    def set_time_ctrl(
        self,
        *,
        sequence: int | None = None,
        nanoseconds: int | None = None,
        seconds: float | None = None,
        timeline: str | None = None,
        play: bool = False,
    ) -> None:
        """
        Set the time control for the viewer.

        Parameters
        ----------
        sequence: int
            The sequence number to set the viewer to.
        seconds: float
            The time in seconds to set the viewer to.
        nanoseconds: int
            The time in nanoseconds to set the viewer to.
        play: bool
            Whether to start playing from the specified time point. Defaults to paused.
        timeline : str
            The name of the timeline to switch to. If not provided, time will remain on the current timeline.

        """
        if sum([sequence is not None, nanoseconds is not None, seconds is not None]) > 1:
            raise ValueError("At most one of sequence, nanoseconds, or seconds may be provided")

        if sequence is not None:
            time = sequence
        elif nanoseconds is not None:
            time = nanoseconds
        elif seconds is not None:
            time = int(seconds * 1e9)
        else:
            time = None

        self._viewer.set_time_ctrl(timeline, time, play)


def notebook_show(
    *,
    width: int | None = None,
    height: int | None = None,
    blueprint: BlueprintLike | None = None,
    recording: RecordingStream | None = None,
) -> None:
    """
    Output the Rerun viewer in a notebook using IPython [IPython.core.display.HTML][].

    Any data logged to the recording after initialization will be sent directly to the viewer.

    Note that this can be called at any point during cell execution. The call will block until the embedded
    viewer is initialized and ready to receive data. Thereafter any log calls will immediately send data
    to the viewer.

    Parameters
    ----------
    width : int
        The width of the viewer in pixels.
    height : int
        The height of the viewer in pixels.
    blueprint : BlueprintLike
        A blueprint object to send to the viewer.
        It will be made active and set as the default blueprint in the recording.

        Setting this is equivalent to calling [`rerun.send_blueprint`][] before initializing the viewer.
    recording:
        Specifies the [`rerun.RecordingStream`][] to use.
        If left unspecified, defaults to the current active data recording, if there is one.
        See also: [`rerun.init`][], [`rerun.set_global_data_recording`][].

    """

    viewer = Viewer(
        width=width,
        height=height,
        blueprint=blueprint,
        recording=recording,  # NOLINT
    )
    viewer.display()
