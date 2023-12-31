use crate::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::exit;
use std::time::Instant;

/// ### private function generate equivalent expressions
/// ### with 1 input expression
/// #### Argument
/// * `cli` - pre-processed command line arguments
/// #### Return
/// * `equiv_expr` - Vec<String> of equivalent expressions
fn generate_exprs(mut cli: Vec<CliDtype>) -> HashSet<String> {
    /* initialize ctx_gr struct and create egraph, skip_ecls, grammar, init_rewrite */
    let input_expr = cli[5].to_string();
    log_info(&format!("Expression: {}\n", input_expr));
    let mut ctx_gr = ContextGrammar::new(input_expr);
    ctx_gr.setup();
    pt_egraph_info(&ctx_gr.egraph);
    let skip_ecls = &ctx_gr.skip_eclasses.clone();
    let grammar = &ctx_gr.grammar.clone();
    let init_exprs = &ctx_gr.init_exprs.clone();

    #[allow(unused_assignments)]
    let mut equiv_exprs: HashSet<String> = HashSet::default();

    loop {
        let start_time = Instant::now();
        extract(&cli, skip_ecls, grammar, init_exprs);
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time).as_secs();
        log_info(&format!("Expression extraction time: {}s\n", elapsed_time));

        unsafe {
            let start_time = Instant::now();
            equiv_exprs = get_global_equiv_exprs().clone();
            let orig_num_exprs = equiv_exprs.len();
            /* post-processing equivalent expressions */
            equiv_exprs = rm_permu(&equiv_exprs);
            let end_time = Instant::now();
            let elapsed_time = end_time.duration_since(start_time).as_secs();
            let num_exprs = equiv_exprs.len();
            log_info(&format!("Expression postprocessing time: {}s\n", elapsed_time));
            log_info(&format!("Total # of expression(s) before postprocessing: {}\n", orig_num_exprs));
            log_info(&format!("Total # of expression(s) after  postprocessing: {}\n", num_exprs));

            if equiv_exprs.len() >= N_EQUIV_EXPRS as usize {
                break;
            }

            log_info("-----------------------------------\n");
            match cli[2] {
                CliDtype::UInt8(ref mut token_limit) => {
                    *token_limit += 2;
                    if *token_limit > MAX_TOKEN_LIMIT {
                        log_info(&format!("Token limit {} reaches max token limit {}.\n", token_limit, MAX_TOKEN_LIMIT));
                        break;
                    }
                    log_info(&format!("Increase token limit to {}\n", token_limit));
                },
                _ => { log_error(&format!("Failed to convert '{:?}' to u8 datatype.\n", cli[3])); },
            }
            match cli[4] {
                CliDtype::UInt16(ref mut time_limit) => {
                    *time_limit += 300;
                    log_info(&format!("Increase time limit to {}\n", time_limit));
                },
                _ => { log_error(&format!("Failed to convert '{:?}' to u16 datatype.\n", cli[4])); },
            }
        }
    }

    return equiv_exprs;
}

/// ### private function to generate equivalent expressions
/// ### with expressions from an input file
/// #### Argument
/// * `cli` - pre-processed command line arguments
/// #### Return
/// * `None`
fn generate_file(cli: &mut Vec<CliDtype>) {
    /* Open the input file and create output file */
    let input_file = match File::options().read(true).write(false).open(&cli[5].to_string()) {
        Ok(input_file) => { input_file },
        Err(e) => {
            log_error(&format!("Failed to open input file '{}'.\n", &cli[5].to_string()));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    };
    let output_file = match File::create(&cli[6].to_string()) {
        Ok(output_file) => { output_file },
        Err(e) => {
            log_error(&format!("Failed to create output file '{}'.\n", &cli[6].to_string()));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    };

    /* Create buffered reader and writer for the input and output files */
    let reader = BufReader::new(&input_file);
    let mut writer = BufWriter::new(&output_file);

    cli.pop();

    for input_expr in reader.lines() {
        /* read 1 expression and write into output file */
        let input_expr = match input_expr {
            Ok(input_expr) => { input_expr },
            Err(e) => {
                log_error(&format!("Failed to read line from input file '{:?}'.\n", input_file));
                log_error(&format!("{}\n", e));
                exit(1);
            },
        };
        match writeln!(writer, "{}", &input_expr.replace(|c| c == '(' || c == ')', "")) {
            Ok(_) => {},
            Err(e) => {
                log_error(&format!("Failed to write input expr '{}' into output file '{:?}'.\n", input_expr, output_file));
                log_error(&format!("{}\n", e));
                exit(1);
            },
        };

        /* start extraction and get equivalent expressions */
        cli[5] = CliDtype::String(input_expr);
        let start_time = Instant::now();
        let equiv_exprs = generate_exprs(cli.clone());
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time).as_secs();
        log_info(&format!("Total run time: {}s\n\n", elapsed_time));

        /* write equivalent expressions into output file */
        for expr in &equiv_exprs {
            match writeln!(writer, "{}", expr) {
                Ok(_) => {},
                Err(e) => {
                    log_error(&format!("Failed to write expr '{}' into output file '{:?}'.\n", expr, output_file));
                    log_error(&format!("{}\n", e));
                    exit(1);
                },
            };
        }
        match writeln!(writer, "") {
            Ok(_) => {},
            Err(e) => {
                log_error(&format!("Failed to write '' into output file '{:?}'.\n", output_file));
                log_error(&format!("{}\n", e));
                exit(1);
            },
        };

        /* flush the output stream */
        match writer.flush() {
            Ok(_) => {},
            Err(e) => {
                log_error(&format!("Failed to flush buffer to output file '{:?}'.\n", output_file));
                log_error(&format!("{}\n", e));
                exit(1);
            },
        }
    }

    /* flush the output stream */
    match writer.flush() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to flush buffer to output file '{:?}'.\n", output_file));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    }

    /* sync all OS-internal metadata to disk */
    match input_file.sync_all() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to sync all OS-internal metadata to '{:?}' in filesystem.\n", input_file));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    }
    match output_file.sync_all() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to sync all OS-internal metadata to '{:?}' in filesystem.\n", output_file));
            log_error(&format!("{}\n", e));
            exit(1);
        },
    }

    /* clean up file descriptors */
    drop(writer);
    drop(input_file);
    drop(output_file);

    return;
}

/// ### public function to start generating equivalent expressions
/// #### Argument
/// * `args` - raw command line arguments
/// #### Return
/// * `None`
pub fn generate() {
    let mut cli = parse_args();

    if cli.len() == 6 {
        let start_time = Instant::now();
        let equiv_exprs = generate_exprs(cli.clone());
        for expr in &equiv_exprs {
            log_info(&format!("{}\n", expr));
        }
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time).as_secs();
        log_info(&format!("Total run time: {}s\n", elapsed_time));
    }
    else { generate_file(&mut cli); }

    return;
}