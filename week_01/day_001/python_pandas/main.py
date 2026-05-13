import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns


def create_visuals() -> None:

    # Load the data
    try:
        df = pd.read_csv("data/GAID_MASTER_V2_COMPILATION_FINAL.csv")
    except FileNotFoundError:
        print("File not found. Please check the path.")
        return

    # Set aesthetic style
    sns.set_theme(style="whitegrid")

    # Create a figure with two distinct charts
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 6))

    # --- Chart 1: The Timeline ---
    # Grouping by Year to see the growth of data entries
    yearly_counts = df.groupby("Year").size()
    sns.lineplot(
        x=yearly_counts.index,
        y=yearly_counts.values,
        ax=ax1,
        color="#2a9d8f",
        marker="o",
        linewidth=2,
    )
    ax1.set_title(
        "AI Readiness Data Volume (1998 - 2025)", fontsize=14, fontweight="bold"
    )
    ax1.set_xlabel("Year")
    ax1.set_ylabel("Number of Records")

    # --- Chart 2: Top Countries ---
    # Getting the top 10 countries by record count
    top_countries = df["Country"].value_counts().head(10)
    sns.barplot(
        x=top_countries.values,
        y=top_countries.index,
        ax=ax2,
        hue=top_countries.index,
        palette="magma",
        legend=False,
    )
    ax2.set_title("Top 10 Countries by Data Presence", fontsize=14, fontweight="bold")
    ax2.set_xlabel("Number of Records")

    plt.tight_layout()

    # Save and display
    plt.savefig("ai_readiness_analysis.png", dpi=300)
    print("Chart saved as 'ai_readiness_analysis.png'")
    plt.show()


if __name__ == "__main__":
    create_visuals()
