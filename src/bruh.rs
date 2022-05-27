let mut net = Network::new(
	// Initial neuron values, pre sigmoid activation.
	vec![
	  vec![
		 1.0, 0.1, 0.6
	  ],
	  vec![
		 0.0, 0.0, 0.0, 0.0, 0.0
	  ],
	  vec![
		 0.0
	  ]
	], 
	// Weights
	vec![
	  // Layer 1
	  vec![
		 vec![
			1.0, 1.0, 1.0
		 ], 
		 vec![
			3.0, 1.3, 1.0
		 ],
		 vec![
			1.0, 1.0, 1.0
		 ],
		 vec![
			1.0, 1.0, 1.0
		 ],
		 vec![
			1.0, 1.0, 1.0
		 ],
	  ], 
	  // Layer 2
	  vec![
		 vec![
			1.0, 1.0, 1.0, 1.0, 1.0
		 ]
	  ]
	], 
	// Biases
	vec![
	  vec![
		 0.0, 0.0, 0.0, 4.0, 0.0
	  ], 
	  vec![
		 0.0
	  ]
	]);