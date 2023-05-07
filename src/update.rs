use num_rational::Ratio;

use crate::app::Widgets;

pub trait UpdatableWidget {
    fn update(&mut self);
    fn get_update_interval(&self) -> Ratio<u64>;
}

pub fn update_widgets(widgets: &mut Widgets, seconds: Ratio<u64>) {
    let mut widgets_to_update: Vec<&mut (dyn UpdatableWidget)> = vec![];

    if let Some(disk) = widgets.disk.as_mut() {
        widgets_to_update.push(disk); 
    }

    if let Some(net) = widgets.net.as_mut() {
        widgets_to_update.push(net);
    }

    if let Some(temp) = widgets.temp.as_mut() {
        widgets_to_update.push(temp);
    } 

    if let Some(cpu) = widgets.cpu.as_mut() {
        widgets_to_update.push(cpu);
    }

    if let Some(mem) = widgets.mem.as_mut() {
        widgets_to_update.push(mem);
    }   

    if let Some(proc) = widgets.proc.as_mut() {
        widgets_to_update.push(proc);
    }

    if let Some(battery) = widgets.battery.as_mut() {
        widgets_to_update.push(battery);
    }

    for widget in widgets_to_update {
        if seconds % widget.get_update_interval() == Ratio::from_integer(0) {
            widget.update();
        }
    }
}
