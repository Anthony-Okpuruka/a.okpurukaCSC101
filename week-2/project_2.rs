fn main() {
	let toshiba:f64 = 450000.0;
	let mac:f64 = 1500000.0;
	let hp:f64 = 750000.0;
	let dell:f64 = 2850000.0;
	let acer:f64 = 250000.0;
	let efx = (toshiba * 2.0)+(mac * 1.0)+(hp * 3.0)+(dell * 3.0)+(acer * 1.0);
	let fx = 10.0; 
	// average
	let x = efx/fx;
	println!("average is {}", x);
}