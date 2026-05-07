import pandas as pd
import streamlit as st

import sqlite_visualize

st.set_page_config(page_title="Rust-SQLite Logger", layout="wide")


@st.cache_resource
def get_db() -> sqlite_visualize.DBConnection:
    return sqlite_visualize.DBConnection("my_database.db")


db = get_db()

db.create_table("logs")
db.create_table("tests")

with st.sidebar:
    st.title("🛠️ Database Admin")
    st.subheader("Manage Tables")

    new_table_name = st.text_input("New Table Name:")
    if st.button("Create Table") and new_table_name:
        db.create_table(new_table_name.strip())
        st.rerun()

    st.divider()

    table_to_drop = st.selectbox("Select Table to Drop:", db.show_tables())
    if st.button("Drop Table"):
        db.delete_table(table_to_drop)
        st.rerun()

st.title("Sqlite Visualize")

all_tables = db.show_tables()
custom_tables = [t for t in all_tables if t not in ["logs", "tests"]]

col1, col2, col3 = st.columns(3)

with col1, st.form("logs_form", clear_on_submit=True):
    st.subheader("Activity Logs")
    log_input = st.text_input("Enter log message:")
    if st.form_submit_button("Add to Logs") and log_input:
        db.add_row("logs", log_input)
        st.toast("Log Entry Added!")

with col2, st.form("tests_form", clear_on_submit=True):
    st.subheader("Test Results")
    test_input = st.text_input("Enter test result:")
    if st.form_submit_button("Add to Tests") and test_input:
        db.add_row("tests", test_input)
        st.toast("Test Result Added!")

with col3:
    if custom_tables:
        with st.form("custom_form", clear_on_submit=True):
            st.subheader("Custom Table Entry")
            selected_custom = st.selectbox("Target Table:", custom_tables)
            custom_input = st.text_input(f"Enter data for {selected_custom}:")

            if st.form_submit_button(f"Add to {selected_custom}") and custom_input:
                db.add_row(selected_custom, custom_input)
                st.toast(f"Added to {selected_custom}!")
    else:
        st.info("Create a custom table in the sidebar to unlock this column.")


st.divider()

st.subheader("Data Explorer")
all_tables = db.show_tables()

if all_tables:
    tabs = st.tabs([f"TABLE {t.upper()}" for t in all_tables])

    for i, table in enumerate(all_tables):
        with tabs[i]:
            try:
                rows = db.get_rows(table)
                if rows:
                    df = pd.DataFrame(rows, columns=["ID", "Entry Content"])

                    search = st.text_input(f"Filter {table}...", key=f"search_{table}")
                    if search:
                        df = df[df["Entry Content"].str.contains(search, case=False)]

                    st.dataframe(df, width="stretch", hide_index=True)
                else:
                    st.info(f"The table '{table}' is currently empty.")
            except Exception as exc:
                st.error(f"Error reading {table}: {exc}")
else:
    st.warning("No tables found. Use the sidebar to create one!")
