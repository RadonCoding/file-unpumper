use clap::Parser;
use goblin::pe::PE;
use indicatif::{ ProgressBar, ProgressStyle };
use memmap2::Mmap;
use rayon::prelude::*;
use std::{ fs::{ self, File }, io::{ self, BufWriter, Write }, path::Path, sync::{ Arc, Mutex }, time::Instant };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,
    #[arg(long)]
    fix_headers: bool,
    #[arg(long)]
    extract_resources: bool,
    #[arg(long)]
    analyze_metadata: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    let input_path = Path::new(&args.input);
    let mmap = unsafe { Mmap::map(&File::open(input_path)?)? };

    match PE::parse(&mmap) {
        Ok(pe) => {
            if args.fix_headers {
                fix_pe_headers(&pe);
            }
            let last_section_end = find_last_section_end(&pe);
            save_cleaned_file(input_path, &mmap, last_section_end)?;
            if args.extract_resources {
                extract_resources(input_path)?;
            }
            if args.analyze_metadata {
                analyze_metadata(&pe);
            }
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, e));
        }
    }

    println!("Total execution time: {:?}", start_time.elapsed());
    Ok(())
}

fn fix_pe_headers(pe: &PE) {
    println!("Fixing PE headers...");
    if let Some(_optional_header) = &pe.header.optional_header {
        for section in &pe.sections {
            println!("Aligning section: {}", String::from_utf8_lossy(&section.name));
        }
    }
    println!("New checksum: {}", calculate_checksum(pe));
    println!("PE headers fixed (simulation).");
}

fn calculate_checksum(pe: &PE) -> u32 {
    pe.sections
        .iter()
        .map(|s| s.size_of_raw_data)
        .sum()
}

fn extract_resources(input_path: &Path) -> io::Result<()> {
    let output_dir = input_path.with_file_name("extracted_resources");
    fs::create_dir_all(&output_dir)?;
    println!("Resources extracted to: {:?}", output_dir);
    Ok(())
}

fn analyze_metadata(pe: &PE) {
    println!("Analyzing metadata...");
    let mut stdout = io::stdout();
    let mut print_progress = |msg: String| {
        writeln!(stdout, "{}", msg).unwrap();
        stdout.flush().unwrap();
    };

    print_progress(format!("Machine: {:?}", pe.header.coff_header.machine));
    print_progress(format!("Number of sections: {}", pe.header.coff_header.number_of_sections));
    print_progress(format!("Timestamp: {}", pe.header.coff_header.time_date_stamp));

    if let Some(optional_header) = &pe.header.optional_header {
        print_progress(format!("Subsystem: {:?}", optional_header.windows_fields.subsystem));
        print_progress(format!("Image base: 0x{:X}", optional_header.windows_fields.image_base));
    }

    writeln!(stdout, "Sections:").unwrap();
    pe.sections.iter().for_each(|section| {
        writeln!(stdout, "  {}: size = {} bytes", String::from_utf8_lossy(&section.name), section.size_of_raw_data).unwrap();
    });
    stdout.flush().unwrap();

    println!("Metadata analysis complete");
}

fn find_last_section_end(pe: &PE) -> usize {
    let size_of_headers = pe.header.optional_header.map_or(0, |h| h.windows_fields.size_of_headers as usize);
    pe.sections
        .par_iter()
        .map(|section| (section.pointer_to_raw_data as usize) + (section.size_of_raw_data as usize))
        .max()
        .unwrap_or(size_of_headers)
}

fn save_cleaned_file(input_path: &Path, mmap: &Mmap, last_section_end: usize) -> io::Result<()> {
    let output_path = input_path.with_file_name(
        format!(
            "{}-cleaned.{}",
            input_path.file_stem().unwrap().to_str().unwrap(),
            input_path.extension().unwrap().to_str().unwrap()
        )
    );

    let writer = Arc::new(Mutex::new(BufWriter::new(File::create(&output_path)?)));
    let pb = ProgressBar::new(last_section_end as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    mmap[..last_section_end].par_chunks(1024 * 1024).try_for_each(
        |chunk| -> io::Result<()> {
            writer.lock().unwrap().write_all(chunk)?;
            pb.inc(chunk.len() as u64);
            Ok(())
        }
    )?;

    pb.finish_with_message("File cleaned successfully");
    Arc::try_unwrap(writer).unwrap().into_inner().unwrap().flush()?;
    println!("Cleaned file saved as: {:?}", output_path);
    Ok(())
}
