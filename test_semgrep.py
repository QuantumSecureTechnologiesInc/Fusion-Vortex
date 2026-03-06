import os

# This is a potential security vulnerability - using eval with user input
def process_user_input(user_data):
    result = eval(user_data)  # Dangerous: arbitrary code execution
    return result

# Another vulnerability - hardcoded password
password = "super_secret_password_123"

def connect_to_db():
    # SQL injection vulnerability
    query = f"SELECT * FROM users WHERE name = '{user_input}'"
    return query
