import pandas as pd
#filtering out some columns and excess data from https://github.com/JerBouma/FinanceDatabase/tree/main/database
df_equities = pd.read_csv('./raw_data/equities.csv')
df_etfs = pd.read_csv('./raw_data/etfs.csv')

markets = ['New York Stock Exchange', 'NYSE MKT', 'NASDAQ Global Select']



filtered_equities = df_equities[df_equities['market'].isin(markets)]
filtered_etfs = df_etfs[df_etfs['market'] == 'us_market']


new_df_equities = filtered_equities[['symbol', 'name']]
new_df_etfs = filtered_etfs[['symbol', 'name']]





new_df_equities.to_csv('./filtered_data/equities.csv', index=False)
new_df_etfs.to_csv('./filtered_data/etfs.csv', index=False)