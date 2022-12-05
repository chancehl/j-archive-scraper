# j-archive-scraper

This tool crawls [j-archive](https://j-archive.com/) to generate Jeopardy question in JSON format.

# Example data

Some initial example data has been written to the `./results/results.json` file that is included in this repository. This data was generated via `$ cargo run -- --outfile ./results/results.json --episode 7050 --iterations 50`.

# cli --help

```
Program to scrape jeopardy question data from j-archive.com

Usage: j-archive-scraper [OPTIONS]

Options:
  -e, --episode <EPISODE_NO>     The episode number to parse (note: if iteratons are applied, this will be the starting episode) [default: 7515]
  -i, --iterations <ITERATIONS>  The number of iterations [default: 1]
  -o, --outfile <OUTFILE>        Where to write the results to
  -h, --help                     Print help information
  -V, --version                  Print version information ./README.md
```