use std::rc::Rc;

use num::{BigUint,BigInt, bigint::{ToBigInt, Sign}};
use wave_insight_lib::data_struct::{Signal, Module};
use yew::prelude::*;

use crate::wave_show::{Settings, ShowType};

#[derive(Clone, Debug, PartialEq)]
pub struct SignalValue {
    points1: String,
    points2: String,
    value: Vec<Html>,
    height: String,
    bool_signal: bool,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SignalValueProps {
    pub module: Rc<Module>,
    pub signal: Rc<Signal>,
    pub bool_signal: bool,
    pub x_axis: f64,
    pub size: f64,
    pub setting: Settings,
}

impl Component for SignalValue {
    type Message = ();
    type Properties = SignalValueProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let x_axis = props.x_axis;
        let size = props.size;
        let show_type = &props.setting.show_type;
        let bitcount = props.signal.size as u32;
        let zero_position = 3;
        let mut points1 = String::new();
        let mut points2 = String::new();
        let mut value: Vec<Html> = vec![];
        if props.bool_signal {
            let mut last: u32 = 0;
            let mut head: u32 = 0;
            let mut head_used = false;
            for d in props.module.value.get(&props.signal.value_key).unwrap() {
                let x = ((d.0 as f64) - x_axis)*size;
                if (0.0..3000.0).contains(&x) {
                    if !head_used {
                        points1.push_str(&format!("{:.2},{} ", 0, zero_position+(1-head)*24));
                        head_used = true;
                    }
                    if d.1 == BigUint::new(vec![1]){
                        points1.push_str(&format!("{:.2},{} ", x, zero_position+24));
                        points1.push_str(&format!("{:.2},{} ", x, zero_position));
                        last = zero_position;
                    }else {
                        points1.push_str(&format!("{:.2},{} ", x, zero_position));
                        points1.push_str(&format!("{:.2},{} ", x, zero_position+24));
                        last = zero_position+24;
                    }
                }else if d.1 == BigUint::new(vec![1]) {
                    head = 1;
                }else {
                    head = 0;
                }
            };
            if !head_used {
                points1.push_str(&format!("{:.2},{} ", 0, zero_position+(1-head)*24));
                last = zero_position+(1-head)*24;
            }
            points1.push_str(&format!("{:.2},{} ", 3000, last));

        }else {
            
            let mut head: BigUint = BigUint::new(vec![0]);
            let mut head_used = true;
            for d in props.module.value.get(&props.signal.value_key).unwrap() {
                let x = ((d.0 as f64) - x_axis)*size;
                if (0.0..3000.0).contains(&x) {
                    if !head_used {
                        head_used = true;
                        points1.push_str(&format!("{:.2},{} ", 0, zero_position+24));
                        points2.push_str(&format!("{:.2},{} ", 0, zero_position));
                        value.push(value_text(0.0, &head, show_type, bitcount));
                    }
                    points1.push_str(&format!("{:.2},{} ", x-2.0, zero_position+24));
                    points1.push_str(&format!("{:.2},{} ", x, zero_position+12));
                    points1.push_str(&format!("{:.2},{} ", x+2.0, zero_position+24));
                    points2.push_str(&format!("{:.2},{} ", x-2.0, zero_position));
                    points2.push_str(&format!("{:.2},{} ", x, zero_position+12));
                    points2.push_str(&format!("{:.2},{} ", x+2.0, zero_position));

                    value.push(value_text(x+2.0, &d.1, show_type, bitcount));
                }else if x < 0.0 {
                    head = d.1.clone();
                    head_used = false;
                }
            };
            if !head_used {
                points1.push_str(&format!("{:.2},{} ", 0, zero_position+24));
                points2.push_str(&format!("{:.2},{} ", 0, zero_position));
                value.push(value_text(0.0, &head, show_type, bitcount));
            }
            points1.push_str(&format!("{:.2},{} ", 3000, zero_position+24));
            points2.push_str(&format!("{:.2},{} ", 3000, zero_position));

            
        }
        Self {
            points1,
            points2,
            value,
            height: "20px".to_string(),
            bool_signal: props.bool_signal,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();
        let x_axis = props.x_axis;
        let size = props.size;
        let show_type = &props.setting.show_type;
        let bitcount = props.signal.size as u32;
        let zero_position = 3;
        let mut points1 = String::new();
        let mut points2 = String::new();
        let mut value: Vec<Html> = vec![];
        if props.bool_signal {
            let mut last: u32 = 0;
            let mut head: u32 = 0;
            let mut head_used = false;
            for d in props.module.value.get(&props.signal.value_key).unwrap() {
                let x = ((d.0 as f64) - x_axis)*size;
                if (0.0..3000.0).contains(&x) {
                    if !head_used {
                        points1.push_str(&format!("{:.2},{} ", 0, zero_position+(1-head)*24));
                        head_used = true;
                    }
                    if d.1 == BigUint::new(vec![1]){
                        points1.push_str(&format!("{:.2},{} ", x, zero_position+24));
                        points1.push_str(&format!("{:.2},{} ", x, zero_position));
                        last = zero_position;
                    }else {
                        points1.push_str(&format!("{:.2},{} ", x, zero_position));
                        points1.push_str(&format!("{:.2},{} ", x, zero_position+24));
                        last = zero_position+24;
                    }
                }else if d.1 == BigUint::new(vec![1]) {
                    head = 1;
                }else {
                    head = 0;
                }
            };
            if !head_used {
                points1.push_str(&format!("{:.2},{} ", 0, zero_position+(1-head)*24));
                last = zero_position+(1-head)*24;
            }
            points1.push_str(&format!("{:.2},{} ", 3000, last));

        }else {
            
            let mut head: BigUint = BigUint::new(vec![0]);
            let mut head_used = true;
            for d in props.module.value.get(&props.signal.value_key).unwrap() {
                let x = ((d.0 as f64) - x_axis)*size;
                if (0.0..3000.0).contains(&x) {
                    if !head_used {
                        head_used = true;
                        points1.push_str(&format!("{:.2},{} ", 0, zero_position+24));
                        points2.push_str(&format!("{:.2},{} ", 0, zero_position));
                        value.push(value_text(0.0, &head, show_type, bitcount));
                    }
                    points1.push_str(&format!("{:.2},{} ", x-2.0, zero_position+24));
                    points1.push_str(&format!("{:.2},{} ", x, zero_position+12));
                    points1.push_str(&format!("{:.2},{} ", x+2.0, zero_position+24));
                    points2.push_str(&format!("{:.2},{} ", x-2.0, zero_position));
                    points2.push_str(&format!("{:.2},{} ", x, zero_position+12));
                    points2.push_str(&format!("{:.2},{} ", x+2.0, zero_position));

                    value.push(value_text(x+2.0, &d.1, show_type, bitcount));
                }else if x < 0.0 {
                    head = d.1.clone();
                    head_used = false;
                }
            };
            if !head_used {
                points1.push_str(&format!("{:.2},{} ", 0, zero_position+24));
                points2.push_str(&format!("{:.2},{} ", 0, zero_position));
                value.push(value_text(0.0, &head, show_type, bitcount));
            }
            points1.push_str(&format!("{:.2},{} ", 3000, zero_position+24));
            points2.push_str(&format!("{:.2},{} ", 3000, zero_position));

        }
        self.points1 = points1;
        self.points2 = points2;
        self.value = value;
        self.bool_signal = props.bool_signal;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.bool_signal {
            html! {
                <svg style="height:30px;width:100%">
                    <polyline points={self.points1.clone()} fill="none" stroke={"rgb(0,255,0)"} />
                </svg>}
        }else {
            html! {
                <svg style="height:30px;width:100%">
                    <polyline points={self.points1.clone()} fill="none" stroke={"rgb(0,255,0)"} />
                    <polyline points={self.points2.clone()} fill="none" stroke={"rgb(0,255,0)"} />
                    {for self.value.clone()}
                </svg> }
        }
    }
}

fn value_text(begin: f64, value: &BigUint, show_type: &ShowType, bitcount: u32) -> Html {
    let zero_position = 3;
    html!{
        <text x={format!("{}",begin)} y={format!("{}",zero_position+21)} fill="rgb(255,255,255)">
            {
                if *show_type==ShowType::Hex {
                    value.to_str_radix(16).to_string()
                }else if *show_type==ShowType::Oct {
                    value.to_str_radix(8).to_string()
                }else if *show_type==ShowType::Bin {
                    value.to_str_radix(2).to_string()
                }else if *show_type==ShowType::UInt {
                    format!("{}",value)
                }else if *show_type==ShowType::SInt {
                    let bound = BigUint::new(vec![2]).pow(bitcount-1);
                    let value_to_sint = if *value >= bound {
                        value.to_bigint().unwrap() - BigInt::new(Sign::Plus,vec![2]).pow(bitcount)
                    }else {
                        value.to_bigint().unwrap()
                    };
                    format!("{}",value_to_sint)
                }else {
                    let value_to_bytes = value.to_bytes_be();
                    let s = match std::str::from_utf8(&value_to_bytes) {
                        Ok(v) => v,
                        Err(_e) => "invalid",//TODO:do not panic
                    };
                    s.to_string()
                }
            }
        </text>
    }
}
