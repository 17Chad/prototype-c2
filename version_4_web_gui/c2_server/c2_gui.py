import pymysql
from nicegui import ui

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

# Function to fetch implants from the MySQL database
def fetch_implants():
    cursor.execute("SELECT * FROM implants")
    result = cursor.fetchall()
    implants = []

    for row in result:
        implant = {
            'implant_name': row[0],
            'implant_id': row[1],
            'hostname': row[2],
            'ip_address': row[3],
            'os': row[4],
            'first_seen': str(row[5]),
            'last_seen': str(row[6])
        }
        implants.append(implant)

    # Calculate height based on the number of rows
    grid.style("min-height: 600px")

    grid.options['rowData'] = implants
    grid.update()

# Function to add a command to the MySQL database
def add_command_to_db(implant_id, command):
    query = "INSERT INTO commands (implant_id, command, status) VALUES (%s, %s, 'pending')"
    cursor.execute(query, (implant_id, command))
    db_conn.commit()
    print(f"Command '{command}' added for implant {implant_id} with status 'pending'.")

# Function to fetch command output from the database
def fetch_command_output(implant_id):
    cursor.execute("SELECT command, command_output, status, timestamp FROM command_history WHERE implant_id=%s", (implant_id,))
    result = cursor.fetchall()
    output_text = "Command History:\n\n"
    for row in result:
        cmd, cmd_output, status, ts = row
        output_text += f"Command: {cmd}\nOutput: {cmd_output}\nStatus: {status}\nTimestamp: {ts}\n\n"
    return output_text

# Function to add a command and update the output display
def add_command():
    implant_id = implant_id_input.value
    command = command_input.value
    add_command_to_db(implant_id, command)
    # Clear the input fields
    implant_id_input.value = ''
    command_input.value = ''
    # Update the output display
    output_display.value = fetch_command_output(implant_id)

# Initialize the AG-Grid for displaying implants
grid = ui.aggrid({
    'defaultColDef': {'flex': 1},
    'columnDefs': [
        {'headerName': 'Implant Name', 'field': 'implant_name'},
        {'headerName': 'Implant ID', 'field': 'implant_id'},
        {'headerName': 'Hostname', 'field': 'hostname'},
        {'headerName': 'IP Address', 'field': 'ip_address'},
        {'headerName': 'OS', 'field': 'os'},
        {'headerName': 'First Seen', 'field': 'first_seen'},
        {'headerName': 'Last Seen', 'field': 'last_seen'},
    ],
    'rowData': [],
    'rowSelection': 'multiple',
}).classes('max-h-40')

# Textarea for displaying command output
output_display = ui.textarea('Output:').classes('max-h-40')

# UI Setup
implant_id_input = ui.input("Implant ID:", placeholder="Enter implant ID")
command_input = ui.input("Command:", placeholder="Enter command to send")

# Button to fetch and display command output
def show_command_output():
    implant_id = implant_id_input.value
    output_display.value = fetch_command_output(implant_id)

ui.button('Fetch Command Output', on_click=show_command_output)

# Fetch and display implants on startup
fetch_implants()

ui.button('Refresh Implants', on_click=fetch_implants)
ui.button('Add Command', on_click=add_command)

ui.run(port=5001)
