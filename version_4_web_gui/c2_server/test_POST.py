import pymysql

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

def add_command(implant_id, command):
    query = "INSERT INTO commands (implant_id, command, status) VALUES (%s, %s, 'pending')"
    cursor.execute(query, (implant_id, command))
    db_conn.commit()
    print(f"Command '{command}' added for implant {implant_id} with status 'pending'.")

implant_id = "implant-1694016203"
command = "ls -al"  # Replace with the command you want to send

add_command(implant_id, command)
