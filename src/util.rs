pub enum TransformDirection {
    Forward,
    Inverse
}

#[macro_export]
macro_rules! discrete_transform {
    ($transform: ident, $input: ident, $direction: ident) => {
        {
            let mut transform = $transform::new();
            let mut output = Vec::new();
            let mut output_temp = Vec::new();
            let mut input = Vec::new();

            for chunks in &$input.iter().chunks(8) {
                {
                    transform.set_input(chunks.map(|x| *x).collect_vec());
                }

                match $direction {
                    TransformDirection::Forward => { input = transform.forward() },
                    TransformDirection::Inverse => { input = transform.inverse() }
                };

                output.extend(input.iter().cloned());
            }

            input.clear();

            for x in 0..8 {
                for y in 0..8 {
                    input.push(output[x + 8 * y]);
                }
                {
                    transform.set_input(input.clone());
                }

                match $direction {
                    TransformDirection::Forward => { output_temp.extend(transform.forward()) },
                    TransformDirection::Inverse => { output_temp.extend(transform.inverse()) }
                };

                input.clear();
            }

            for x in 0..8 {
                for y in 0..8 {
                    output[x + 8 * y] = output_temp[x * 8 + y];
                }
            }

            output
        }
    };
}