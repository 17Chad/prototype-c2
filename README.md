**Prototype C2**

*CLEARTEXT IMPLANT COMMS - NOT FOR PRODUCTION*

Correctly runs shell commands using "sh -c $cmd" for now. You can see the network diagram, however the gist is the rust implant will continously beacon to the python c2_app.py and see if there are commands to execute. The python c2_app.py checks the mysql database for commands pending to be executed (this could easily be turned into a web gui button/pseudo terminal). Then the python c2_app.py will send a GET to the database, grab the command, send it to the implant to be executed, and on return, the implant output will be given to the python c2_app.py and stored inside the mysql database. All of this is clear text and just to show proof of concept and learn a little bit of rust and have fun. 
**Note:** Need to perfect the "download/get" and "upload/push" functionality. For now, sentinel characters are appended to the data to show when get/push data is completed on the wire so the connection can be closed.  

**Getting Started**

```
1. MySQL needs to be running    # listening 0.0.0.0:3306 
2. python3 c2_app.py            # listening on 0.0.0.0:5000
3. cargo run                    # connects to C2, callsback to 127.0.0.1:5000
4. test_POST.py                 # puts a command in the database so when the implant calls in, the python c2_app.py pulls from the mysql database then tells the rust implant what to execute, waits for output, then stores the results in the database. 
```


**I am using dbeaver-ce and here are the MySQL tables/commands**

```
CREATE TABLE command_history (
    id INT AUTO_INCREMENT PRIMARY KEY,
    implant_id VARCHAR(255) NOT NULL,
    command VARCHAR(255) NOT NULL,
    command_output MEDIUMTEXT NOT NULL,
    status ENUM('success', 'failure') NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```


```
CREATE TABLE implants (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    last_seen DATETIME,
    status ENUM('online', 'offline')
);
```


```
CREATE TABLE commands (
    id INT AUTO_INCREMENT PRIMARY KEY,
    implant_id INT NOT NULL,
    command TEXT NOT NULL,
    FOREIGN KEY (implant_id) REFERENCES implants(id)
);
```


**Contributing**
Guidelines for how to contribute to the project, including information on how to submit bug reports or feature requests, and how to submit pull requests.

**Acknowledgements**
Any acknowledgements or thanks to individuals or organizations who have contributed to the project, or whose work has inspired the project.
