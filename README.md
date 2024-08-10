## ndate: Bikram Sambat Date Converter

ndate is a command-line tool for displaying and converting dates between the Bikram Sambat (Vikram Samvat) and Gregorian calendars. It's a simple utility designed for easy interaction with these two calendar systems.

### Features

-   Display the current Bikram Sambat date.
-   Convert dates between Bikram Sambat and Gregorian calendars.
-  Supports very long date range.

### Installation

There are two installation methods available: using a Debian package or compiling from source.

#### From Debian Package

1.  **Download the latest `.deb` file** from the [releases](https://github.com/khumnath/ndate/releases) page.
2.  **Install the package** using `dpkg`:

Bash

```
sudo dpkg -i ndate_1.0.0_amd64.deb

```

Replace `ndate_1.0.0_amd64.deb` with the filename of the downloaded package.


#### From Source

1.  Clone the repository:

Bash
```
git clone https://github.com/khumnath/ndate.git
cd ndate
```

2.  Install dependencies:

Bash

```
sudo apt-get update
sudo apt-get install -y cmake build-essential debhelper devscripts

```

* packages debhelper  and devscripts are for building debian installer package.

3.  Build and install:

Bash

```
mkdir build
cd build
cmake ..
make
sudo make install

```
* if need to build deb package, run ``` dpkg-buildpackage -b``` after ```make```


### Usage

Once installed, you can use ndate from the command line. Here are some basic commands:

-   Display the current Bikram Sambat date:

Bash

```
ndate
```
output:
![Screenshot_select-area_20240810182013](https://github.com/user-attachments/assets/7b0f2e84-ec09-44a0-9edc-37bd46a682c4)
* Convert to Bikram Sambat
```
 ndate --conv --tobs 2024 8 10
```
output:
![Screenshot_select-area_20240810182157](https://github.com/user-attachments/assets/474ff597-829e-4f48-8d16-079673f902fd)


* Convert a Bikram Sambat date to Gregorian:
```
ndate --conv --toad 2081 4 26  

```
output:
![Screenshot_select-area_20240810182318](https://github.com/user-attachments/assets/308500ee-15dc-42a3-a1d5-9d5d729b267b)


### ** Replace the date strings with the date you want to convert.

## Contributing

Contributions are welcome! If you have any bug reports or feature requests, please open an issue on the GitHub [repository](https://github.com/khumnath/ndate).

Here's a quick guide to contributing code:

1.  Fork the repository.
3.  Create a new branch for your changes.
5.  Commit your changes and push them to your fork.
7.  Open a pull request against the main branch of the original repository.
    

### License

ndate is released under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html). See the LICENSE file for more details.

### Contact

For any inquiries, you can reach out to [khumnath](https://khumnath.com.np) cg.