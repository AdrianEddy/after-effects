use after_effects as ae;
use ae::register_plugin;

#[derive(Eq, PartialEq, Hash)]
enum Params { Slider }

#[derive(Default)]
struct Plugin { }

#[derive(Default)]
struct Instance { }

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool { true }

    fn params_setup(&self, params: &mut ae::Parameters<Params>) -> Result<(), Error> {
        params.add_param(Params::Slider, "Opacity", *ae::FloatSliderDef::new()
            .set_slider_min(0.0)
            .set_slider_max(100.0)
            .set_valid_min(0.0)
            .set_valid_max(100.0)
        );
        Ok(())
    }
    fn handle_command(&self, _: ae::Command, _: ae::InData, _: ae::OutData) -> Result<(), ae::Error> { Ok(()) }
}

impl AdobePluginInstance for Instance {
    fn flatten(&self) -> Result<Vec<u8>, Error> { Ok(Vec::new()) }
    fn unflatten(_: &[u8]) -> Result<Self, Error> { Ok(Self { }) }

    fn render(&self, in_data: ae::InData, in_layer: &ae::Layer, out_layer: &mut ae::Layer, params: &ae::Parameters<Params>) -> Result<(), ae::Error> {
        let slider_value = if let Some(ae::Param::FloatSlider(slider)) = params.get(Params::Slider) {
            slider.value()
        } else {
            0.0
        };

        let extent_hint = in_data.extent_hint();

        in_layer.iterate(out_layer, 0, extent_hint.top - extent_hint.bottom, extent_hint, |_x: i32, _y: i32, mut pixel: ae::Pixel| -> Result<ae::Pixel, Error> {
            pixel.alpha = (pixel.alpha as f64 * slider_value / 100.0) as u8;

            Ok(pixel)
        })?;
        Ok(())
    }

    fn handle_command(&mut self, _: ae::Command, _: ae::InData, _: ae::OutData) -> Result<(), ae::Error> { Ok(()) }
}

register_plugin!(Plugin, Instance, Params);