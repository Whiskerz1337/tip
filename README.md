# tip 
#### A Simple Linux CLI Tool for Saving and Managing Target IP Addresses

Tip is a command line interface (CLI) tool written in Rust that simplifies managing multiple target IP addresses by storing them in a file and allowing you to easily add, remove, list, and update targets. It also provides a shell function to source the targets into your local variables.

Normal variable assignment is possible in Linux by default, however these are lost on reboot and when creating new shell instances. Personally I regularly find myself with multiple terminal windows open running different scans, and having to retype (and remember) an IP address over and over again. This tool is primarly designed to aid penetration testers and ethical hackers, however will also be useful for network engineers/admins whilst testing connectivity and troubleshooting.

The tool works by storing key-value pairs as {target_name}={IP}, which can be added via ```tip add cloudflare 1.1.1.1```. Once added, targets are immediately accessible as a variable using the following syntax: 

```$cloudflare```

To pass this into any other tool, we can simply use this variable in place of an IP address, so: 

```bash
ping $cloudflare
```
```bash
nmap $cloudflare
```

Currently supported shells:

* bash
* zsh

# Basic Installation

1. Download the latest release ```wget https://github.com/Whiskerz1337/tip/releases/download/v2.0.1/tip_v2.0.1.zip```
2. unzip the folder in the desired location
3. cd into the new folder
4. run ```./tip install```
5. Restart the shell or source the shell config file

You should now be able to access tip from any location.

# Usage

##### Adding a new target to the list
```bash
tip add {target_name} {IP}
```

##### Accessing a target IP as an local variable
```bash
echo ${target_name}
```
```bash
nmap -sV -sC ${target_name}
```

##### Updating an existing target's IP - with update confirmation
```bash
tip add {existing_name} {IP}
```

##### Removing a target from the list
```bash
tip remove {name}
```

##### Display all current targets
```bash
tip list
```

##### Purge the target list
```bash
tip purge
```

## Storing commands

As a side note, tip can also be used to store and execute difficult to remember commands, for example:

```bash
tip add findsuid 'find / -perm -u=s -type f 2>/dev/null'
```

From here we can simply ```echo $findsuid``` to print out the string, or we can directly execute the command via:

```bash
eval $findsuid
```

# Contributing
If you find any issues, feel free to report them on the GitHub repository. Pull requests are also welcome.

# License
This project is licensed under the GNU General Public License.
