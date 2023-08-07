**Prototype C2**

*CLEARTEXT IMPLANT COMMS - NOT FOR PRODUCTION*
Correctly runs commands, can cat etc. Only "sh -c $cmd" for now on embedded device. 
Need to perfect the "download/get" and "upload/push" functions. For now, sentinel characters are appended to the data to show when a get/push is done. 

**Getting Started**

```
1. MySQL needs to be running    # listening 0.0.0.0:3306 
2. python3 c2_app.py            # listening on 0.0.0.0:5000
3. cargo run                    # connects to C2, callsback to 127.0.0.1:5000
4. test_POST.py                 # puts a command in the database so when the implant calls in, the python c2_app.py pulls from the mysql database then tells the rust implant what to execute, waits for output, then stores the results in the database. 
```


**dbeaver-ce MySQL tables**
``
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
