use crate::*;

define_suite!(SequenceInfoSuite, PrSDKSequenceInfoSuite, kPrSDKSequenceInfoSuite, kPrSDKSequenceInfoSuiteVersion);

#[derive(Clone, Copy, Debug, Default)]
pub struct ImmersiveVideoVRConfiguration {
    pub projection_type: pr_sys::PrIVProjectionType,
    pub frame_layout: pr_sys::PrIVFrameLayout,
    pub horizontal_captured_view: u32,
    pub vertical_captured_view: u32,
}

impl SequenceInfoSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    /// Get the video frame size of the sequence.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns the size of the sequence video frame.
    pub fn get_frame_rect(&self, timeline_id: pr_sys::PrTimelineID) -> Result<pr_sys::prRect, Error> {
        call_suite_fn_single!(self, GetFrameRect -> pr_sys::prRect, timeline_id)
    }
    /// Get the aspect ratio of the sequence.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns the aspect ratio numerator and denominator.
    pub fn get_pixel_aspect_ratio(&self, timeline_id: pr_sys::PrTimelineID) -> Result<(u32, u32), Error> {
        let mut num = 0;
        let mut den = 0;
        call_suite_fn!(self, GetPixelAspectRatio, timeline_id, &mut num, &mut den)?;
        Ok((num, den))
    }
    /// Get the framerate of the sequence.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns the framerate in ticks.
    pub fn get_frame_rate(&self, timeline_id: pr_sys::PrTimelineID) -> Result<pr_sys::PrTime, Error> {
        call_suite_fn_single!(self, GetFrameRate -> pr_sys::PrTime, timeline_id)
    }
    /// Get the field type of the sequence.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns the field type.
    pub fn get_field_type(&self, timeline_id: pr_sys::PrTimelineID) -> Result<pr_sys::prFieldType, Error> {
        call_suite_fn_single!(self, GetFieldType -> pr_sys::prFieldType, timeline_id)
    }
    /// Get the zero point of the sequence.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns start time of the sequence.
    pub fn get_zero_point(&self, timeline_id: pr_sys::PrTimelineID) -> Result<pr_sys::PrTime, Error> {
        call_suite_fn_single!(self, GetZeroPoint -> pr_sys::PrTime, timeline_id)
    }
    /// Returns if the sequence timecode is drop or non drop.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns if the sequence timecode is dropframe
    pub fn get_timecode_drop_frame(&self, timeline_id: pr_sys::PrTimelineID) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, GetTimecodeDropFrame -> i32, timeline_id)? != 0)
    }
    /// Returns if the sequence has the proxy flag set.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns if the sequence is in proxy mode
    pub fn get_proxy_flag(&self, timeline_id: pr_sys::PrTimelineID) -> Result<bool, Error> {
        Ok(call_suite_fn_single!(self, GetProxyFlag -> i32, timeline_id)? != 0)
    }
    /// Returns the VR Video settings of the specified sequence.
    /// * `timeline_id` - The timeline instance data.
    ///
    /// Returns [`ImmersiveVideoVRConfiguration`]:
    /// * `projection_type` - The type of projection the specified sequence is using.
    /// * `frame_layout` - The type of frame layout the specified sequence is using.
    /// * `horizontal_captured_view` - How many degrees of horizontal view is captured in the video stream (up to 360).
    /// * `vertical_captured_view` - How many degrees of vertical view is captured in the video stream (up to 180).
    pub fn get_immersive_video_vr_configuration(&self, timeline_id: pr_sys::PrTimelineID) -> Result<ImmersiveVideoVRConfiguration, Error> {
        let mut conf = ImmersiveVideoVRConfiguration::default();
        call_suite_fn!(self, GetImmersiveVideoVRConfiguration, timeline_id, &mut conf.projection_type, &mut conf.frame_layout, &mut conf.horizontal_captured_view, &mut conf.vertical_captured_view)?;
        Ok(conf)
    }
    /// Returns the identifier of the sequence working color space
    /// * `timeline_id` - The timeline instance data.
    ///
    /// Returns PrSDKColorSpaceID with working color space identifier
    pub fn get_working_color_space(&self, timeline_id: pr_sys::PrTimelineID) -> Result<pr_sys::PrSDKColorSpaceID, Error> {
        call_suite_fn_single!(self, GetWorkingColorSpace -> pr_sys::PrSDKColorSpaceID, timeline_id)
    }
    /// Get the HDR graphics white luminance value of the sequence in nits.
    /// * `timeline_id` - the timeline instance data
    ///
    /// Returns HDR graphics white luminance value of the sequence in nits.
    pub fn get_graphics_white_luminance(&self, timeline_id: pr_sys::PrTimelineID) -> Result<u32, Error> {
        call_suite_fn_single!(self, GetGraphicsWhiteLuminance -> u32, timeline_id)
    }
}

