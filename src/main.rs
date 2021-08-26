#![warn(clippy::pedantic, clippy::nursery)]

fn main() {
    // get maximum number from arguments and calculate its length in chars (for padding)
    let (num, pad_num): (usize, usize) = {
        let raw = std::env::args().nth(1);
        let input = raw.as_deref().map(str::trim);
        input
            .and_then(|s| s.parse().ok())
            .zip(input.map(|s| s.chars().count()))
            .unwrap_or((255, 3))
    };

    // # of bit places in maximum number
    let pad_bits = (usize::BITS - num.leading_zeros()) as usize;

    {
        use std::io::{stdout, BufWriter, Write};
        let stdout = stdout();
        let handle = stdout.lock();

        let per_line: usize = {
            // get terminal width or default to 80 cols
            let cols = {
                #[cfg(target_os = "windows")]
                {
                    terminal_size::terminal_size_using_handle(
                        std::os::windows::io::AsRawHandle::as_raw_handle(&handle),
                    )
                    .map_or(80, |(terminal_size::Width(w), _)| w as usize)
                }
                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    terminal_size::terminal_size_using_fd(std::os::unix::io::AsRawFd::as_raw_fd(
                        &handle,
                    ))
                    .map_or(80, |(terminal_size::Width(w), _)| w as usize)
                }
            };

            // count how many numbers can be put on a line
            let mut acc = pad_num + pad_bits + 1;
            let mut i = 0;
            while acc < cols {
                acc += pad_num + pad_bits + 3;
                i += 1;
            }
            i
        };

        let mut handle = BufWriter::with_capacity(1024 * 16 * 2, handle);
        for mut line in (0..=num)
            .step_by(per_line)
            .map(|n| n..=num.min(n + per_line - 1))
        {
            use yansi::Paint;
            if let Some(n) = line.next() {
                write!(
                    handle,
                    "{2:>0$} {3:0>1$b}",
                    pad_num,
                    pad_bits,
                    Paint::yellow(n),
                    n
                )
                .expect("failed to write to stdout");
            }
            for n in line {
                write!(
                    handle,
                    "  {2:>0$} {3:0>1$b}",
                    pad_num,
                    pad_bits,
                    Paint::yellow(n),
                    n
                )
                .expect("failed to write to stdout");
            }
            writeln!(handle).expect("failed to write to stdout");
        }
        handle.flush().expect("failed to flush stdout");
    }
}
