import streamlit as st
import pandas as pd

import sqlite_visualize

db = sqlite_visualize.DBConnection("my_database.db")

# Create a new table
db.create_table("logs")
print("Tables after create:", db.show_tables())

# Delete the table
# db.delete_table("logs")
# print("Tables after delete:", db.show_tables())

# Streamlit

df = pd.DataFrame(db.show_tables(), columns=["id"])

st.dataframe(df)
