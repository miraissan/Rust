
struct Body{
    weight: f64,
    height: f64,
}
fn main() {
  let ichiro =  Body{weight: 80.0,height:165.0};
  let jiro   =  Body{weight:65.0,height:170.0};
  println!("Ichiro={:.1}",clac_bmi(&ichiro));
  println!("jiro={:.1}",clac_bmi(&jiro));
}

fn clac_bmi(body: &Body) -> f64{
  let h = body.height / 100.0;
  body.weight / h.powf(2.0)
}
