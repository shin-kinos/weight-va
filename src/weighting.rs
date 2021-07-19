
use std::f64;

pub fn weight_va( seq_list : &Vec<String> ) -> Vec<f64>
{
	//Number of the sequences and sites.
	let num_seq  : usize = ( *seq_list ).len();

	let mut weight_list : Vec<f64> = vec![ 0.0; num_seq ];

	for i in 0 .. num_seq {
		/*
		Calculate pairwise distance by counting differd symbols.
		seq_pair_1 = One pairwised sequence.
		seq_pair_2 = The other pairwised one.
		num_diff   = Number of the differences in pairwised sequences.
		*/
		let seq_pair_1 : &String = &( seq_list[ i ] );
		//println!( "seq_pair_1 : {}", seq_pair_1 );
		for j in 0 .. num_seq {
			if i != j {
				let seq_pair_2 : &String = &( seq_list[ j ] );
				//println!( "seq_pair_2 : {}", seq_pair_2 );
				let num_diff : usize = count_diff( seq_pair_1, seq_pair_2 );
				weight_list[ i ] += num_diff as f64;
			}
		}
		//println!( "" );
	}

	//println!( "Weights : {:?}", weight_list );

	//Normalize the weight factors so that sum is 1.
	weight_list = normalize( &weight_list );
	//println!( "Normalized weights : {:?}", weight_list );

	let sum_norm_weight : f64 = ( weight_list ).iter().sum();
	println!( "Sum of weights : {}", sum_norm_weight );

	weight_list
}


fn count_diff( seq_1 : &String, seq_2 : &String ) -> usize
{
	let num_seq : usize = ( *seq_1 ).len();

	let seq_1_vec : Vec<char> = ( *seq_1 ).chars().collect();
	let seq_2_vec : Vec<char> = ( *seq_2 ).chars().collect();

	let mut counter : usize = 0;
	for i in 0 .. num_seq {
		/*
		Count the number of different sybols.
		seq_1_vec = One pairwised sequence.
		seq_2_vec = The other pairwised one.
		counter   = Counter.
		*/
		if seq_1_vec[ i ] != seq_2_vec[ i ] {
			counter += 1;
		}
	}

	counter
}

fn normalize( diff_list : &Vec<f64> ) -> Vec<f64>
{
	let len_list : usize = ( *diff_list ).len();

	let mut weight_norm : Vec<f64> = Vec::new();

	let sum : f64 = ( *diff_list ).iter().sum();

	for i in 0 .. len_list {
		/*
		Normalize the weight factors so that sum is 1.
		val_norm  = Normalized values.
		diff_list = Numerator (weights). 
		sum       = Denominator (Sum of weights).
		*/
		let val_norm : f64 = ( *diff_list )[ i ] / sum;
		weight_norm.push( val_norm );
	}

	weight_norm
}
