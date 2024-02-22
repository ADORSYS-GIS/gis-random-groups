# PROJECT NAME
Random Group Generator

    The Random group generator is a CLI application developed in Rust that allows you to efficiently generate random groups for various purposes. Whether you are a teacher organizing students, a project manager assigning team members, this application simplifies the process.

    This CLI tool generates groups using a csv file which have to be added while running the application and equally specifying the number of groups.


# CONTENT

    - Description
    - Feature (s)
    - Installation
    - Usage
    - Contributing
    - License


# FEATURE (S)

    - Assign students to specific groups.


# INSTALLATION

    - Install Rust using the command:
        ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

    - Clone repository using:
        ```git clone git@github.com:ADORSYS-GIS/gis-random-groups.git```

    - Change into the project directory using:
        ```cd gis-random-groups/src/```

    - Build the project using Cargo:
        ```cargo build```


# USAGE

    - Assign students to groups:
        To assign students to a group you will need to load a csv file while precising the number of groups as seen below:

            ```cargo run -- -f <path-to-csv-file> -n <number-of-groups>```

# Breakdown of command.

    - cargo run: This is the command to build and run your Rust project using Cargo.
    - '--' : It is used to specify the arguments that will be passed to the Rust Program.
    - -f: This is an argument passed to your Rust program. It specifies the path to the CSV file you want to process. 
    - n: It specifies the number of groups you want to generate. 


# Contributing

    Contribution to improve or add new features are welcome. All you need to do is to just fork the repository https://github.com/ADORSYS-GIS/gis-random-groups and make changes, then submit a pull request.


# LICENSE

    - This project is licensed under the MIT License.