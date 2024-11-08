import pandas as pd
import requests
from bs4 import BeautifulSoup

def wiki_table_to_csv(url, table_index=0, output_file='output.csv'):
    """
    Convert a Wikipedia table to CSV file
    
    Parameters:
    url (str): Full Wikipedia article URL
    table_index (int): Index of the table to convert (0 for first table)
    output_file (str): Name of output CSV file
    
    Returns:
    pandas.DataFrame: The extracted table data
    """
    # Fetch the Wikipedia page
    response = requests.get(url)
    soup = BeautifulSoup(response.text, 'html.parser')
    
    # Find all tables in the page
    tables = soup.find_all('table', class_='wikitable')
    
    if not tables:
        raise ValueError("No tables found on the page")
    
    if table_index >= len(tables):
        raise ValueError(f"Table index {table_index} is out of range. Only {len(tables)} tables found.")
    
    # Convert the selected table to a pandas DataFrame
    df = pd.read_html(str(tables[table_index]))[0]
    
    # Handle multi-level columns if they exist
    if isinstance(df.columns, pd.MultiIndex):
        # Join multi-level column names with underscore
        df.columns = [' '.join(col).strip() for col in df.columns.values]
    
    # Clean any remaining whitespace in column names
    df.columns = [col.strip() for col in df.columns]
    
    # Save to CSV
    df.to_csv(output_file, index=False)
    print(f"Saved table to {output_file}")
    return df

# Example usage:
if __name__ == "__main__":
    url = "https://en.wikipedia.org/wiki/List_of_cities_in_Italy"
    df = wiki_table_to_csv(url, table_index=0, output_file='italian-cities.csv')
    print("\nTable preview:")
    print(df.head())
