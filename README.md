# nepdate-cli

`nepdate-cli` is a simple command-line program built using the `bikram` library to convert dates between Bikram Sambat (Nepali calendar) and Gregorian dates.

## Features

- Convert Gregorian dates to Bikram Sambat.
- Convert Bikram Sambat dates to Gregorian.

## Installation and Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/) programming language
- Cargo (Rust package manager). install instruction for rust [HERE](https://www.rust-lang.org/tools/install).

### Clone the Repository

First, clone the repository:

```bash
git clone --branch rust https://github.com/khumnath/nepdate-cli.git
cd nepdate-cli
```

### Build the Program

1. Make sure to give execution permissions to the `build.sh` script:
    ```bash
    chmod +x build.sh
    ```

2. Run the `build.sh` script to build the program and set up the build folder:
    ```bash
    ./build.sh
    ```

This script will:
- Compile the program in release mode and store the output in the `target` directory.
- Display the path to the executable.
- Test the program by running it.

The executable path should be printed in the terminal, and after 2 seconds, the program will be test run automatically.

## Usage

After building the program, you can use it to convert dates between the two calendar systems.

### Convert to Nepali Date (Bikram Sambat):

```
./target/release/nepdate-cli --conv --tobs <year> <month> <day>
```

Example:
```
./target/release/nepdate-cli --conv --tobs 2024 10 18
```

### Convert to Gregorian Date:

```
./target/release/nepdate-cli --conv --toad <year> <month> <day>
```

Example:
```
./target/release/nepdate-cli --conv --toad 2081 6 1
```


### License

nepdate-cli is released under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html). See the LICENSE file for more details.

### Contact

For any inquiries, you can reach out to [khumnath](https://khumnath.com.np) cg.
