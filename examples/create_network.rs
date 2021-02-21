use n2::network::NetworkBuilder;

fn main() {
    let network = NetworkBuilder::new()
        .input_layer::<5>()
        .hidden_layer::<15>()
        .hidden_layer::<10>()
        .output_layer::<5>()
        .build();

    println!("{:#?}", network);
}
