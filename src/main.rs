use rand::Rng;

struct Network {
  layers: Vec<Vec<f64>>,
  weights: Vec<Vec<Vec<f64>>>,
  biases: Vec<Vec<f64>>
}

impl Network {
  // fn new(
  //     l: Vec<Vec<f64>>, 
  //     w: Vec<Vec<Vec<f64>>>, 
  //     b: Vec<Vec<f64>>
  //   ) -> Network {
  //   Network {
  //     layers: l,
  //     weights: w,
  //     biases: b
  //   }
  // }

  fn new_randomized(inputs: u32, outputs: u32, hidden_layers: u32, hidden_layer_depth: u32) -> Network {
    let mut l = Vec::new();
    let mut w = Vec::new();
    let mut b = Vec::new();

    // Input layer
    l.push(zeros(inputs));
    let mut input_weigths = Vec::new();
    for _ in 0..hidden_layer_depth {
      input_weigths.push(random_values(inputs));
    }
    w.push(input_weigths);

    // Hidden layers
    for _ in 0..hidden_layers {
      l.push(zeros(hidden_layer_depth));
      let mut hidden_weights = Vec::new();
      for __ in 0..hidden_layer_depth {
        hidden_weights.push(random_values(hidden_layer_depth))
      }
      w.push(hidden_weights);
      b.push(random_values(hidden_layer_depth));
    }

    // Output layer
    let mut output_weights = Vec::new();
    for _ in 0..outputs {
      output_weights.push(random_values(hidden_layer_depth));
    }
    b.push(random_values(outputs));
    w.push(output_weights);
    l.push(zeros(outputs));

    Network {
      layers: l,
      weights: w,
      biases: b
    }
  }

  fn forward(&mut self) {
    for l in 1..self.layers.len() {
      for n in 0..self.layers[l].len() {
        let mut sum = 0.;
        for w in 0..self.weights[l-1][n].len() {
          sum += self.weights[l-1][n][w] * self.layers[l-1][w];
        }
        // println!("sum: {:.2}", sig(sum));
        self.layers[l][n] = sig(sum+self.biases[l-1][n]);
      }
    }
  }

  fn backpropogate(&mut self, expected_values: Vec<f64>) {
    print!("\n");
    for i in 0..self.layers[self.layers.len()-1].len() {
      print!("[ {:.3} ] ", expected_values[i]-self.layers[self.layers.len()-1][i]);
    }
    print!("\n");
  }

  fn set_inputs(&mut self, inputs: Vec<f64>) {
    self.layers[0] = inputs;
  }

  fn print_neuron_values(&self) {
    for i in 0..self.layers.len() {
      for j in 0..self.layers[i].len() {
        print!("( {:.3} )", self.layers[i][j]);
      }
      print!("\n");
    }
  }


}

fn zeros(size: u32) -> Vec<f64> {
  let mut v = Vec::new(); 
  for _ in 0..size {
    v.push(0.0f64);
  }
  v
}

fn random_values(size: u32) -> Vec<f64> {
  let mut v = Vec::new();
  let mut rng = rand::thread_rng();
  for _ in 0..size {
    v.push(rng.gen::<f64>()-rng.gen::<f64>());
  }
  v
}

fn sig(val: f64) -> f64 {
  1./(1.+((2.718281828459045 as f64).powf(-val)))
}

fn main() {
  let mut net = Network::new_randomized(2, 2, 10, 5);
  net.set_inputs(vec![0.3, 0.7, 0.1, 0.5]);
  net.forward();
  net.print_neuron_values();
  net.backpropogate(vec![0.4, 0.3]);
}