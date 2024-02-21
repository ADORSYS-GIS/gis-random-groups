# PROJECT NAME
Random Group Generator

    The Random group generator is an application developed in Rust that allows you to efficiently generate random groups for various purposes. Whether you are a teacher organizing students, a project manager assigning team members, this application simplifies the process.

    With the Random group generator you can add students or individuals to the application and then generate randomized groups based on your desired group size.


# CONTENT

    - Description
    - Features
    - Installation
    - Usage
    - Contributing
    - License


# FEATURES
    
    - Add, Update and Remove students from the application.

    - Generate random groups of students.

    - Assign students to  specific groups.


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

    - Adding a new student to the application:
        ```cargo run add-student <student-name>```

    - Assigning random groups:
        ```cargo run generate-groups```

    - Assign student to a specific group:
        ```cargo run assign-group <student-name> <group-number>```


# Contributing

    Contribution to improve or add new features are welcome. All you need to do is to just fork the repository https://github.com/ADORSYS-GIS/gis-random-groups and make changes, then submit a pull request.


# LICENSE

    - This project is licensed under the MIT License.