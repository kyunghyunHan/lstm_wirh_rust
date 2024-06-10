use candle_nn::{LSTM,RNN};
use candle_core::{Result,Device,Tensor,test_utils::to_vec2_round,DType};
pub struct LstmModel{
   hidden_layer_size :i32,
   lstm:LSTM,
}

impl LstmModel{
    pub fn new(){

    }
}
fn main()->Result<()>{
    let cpu = &Device::Cpu;
    let w_ih = Tensor::arange(0f32, 24f32, cpu)?.reshape((12, 2))?;
    let w_ih = w_ih.cos()?;
    let w_hh = Tensor::arange(0f32, 36f32, cpu)?.reshape((12, 3))?;
    let w_hh = w_hh.sin()?;
    let b_ih = Tensor::new(
        &[-1f32, 1., -0.5, 2., -1., 1., -0.5, 2., -1., 1., -0.5, 2.],
        cpu,
    )?;
    let b_hh = b_ih.cos()?;
    let tensors: std::collections::HashMap<_, _> = [
        ("weight_ih_l0".to_string(), w_ih),
        ("weight_hh_l0".to_string(), w_hh),
        ("bias_ih_l0".to_string(), b_ih),
        ("bias_hh_l0".to_string(), b_hh),
    ]
    .into_iter()
    .collect();
    let vb = candle_nn::VarBuilder::from_tensors(tensors, DType::F32, cpu);
    let lstm = candle_nn::lstm(2, 3, Default::default(), vb)?;
    let mut state = lstm.zero_state(1)?;
    for inp in [3f32, 1., 4., 1., 5., 9., 2.] {
        let inp = Tensor::new(&[[inp, inp * 0.5]], cpu)?;
        state = lstm.step(&inp, &state)?
    }
    let h = state.h();
    let c = state.c();
    assert_eq!(to_vec2_round(h, 4)?, &[[0.9919, 0.1738, -0.1451]]);
    assert_eq!(to_vec2_round(c, 4)?, &[[5.725, 0.4458, -0.2908]]);
    Ok(())

}