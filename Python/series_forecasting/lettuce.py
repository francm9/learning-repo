import pandas as pd
import matplotlib.pyplot as plt
from skforecast.ForecasterAutoreg import ForecasterAutoreg
from lightgbm import LGBMRegressor
from skforecast.model_selection import bayesian_search_forecaster
import numpy as np
from sklearn.metrics import mean_squared_error
from sklearn.tree import DecisionTreeRegressor

train_start = '2023-08-03'
train_end = '2023-08-31'
test_start = '2023-09-01'
test_end = '2023-09-16'

data = pd.read_csv('lettuce_dataset.csv', encoding='latin-1')

# Quedarse con el plant_id 1
data = data[data['Plant_ID'] == 1]
data['Date'] = pd.to_datetime(data['Date'], format='%m/%d/%Y')
data.drop(['Plant_ID'], axis=1)
data.set_index('Date',inplace=True)
data.sort_index(inplace=True)

#Cambiar la frecuencia a diaria y rellenar los valores faltantes
data = data.asfreq('D', method='bfill')

forecaster = ForecasterAutoreg(
    regressor = DecisionTreeRegressor(random_state=123),
    lags = 10
)

forecaster.fit(
    y=data.loc[train_start:train_end, 'Humidity (%)']
)

res = forecaster.predict(
    steps=len(data.loc[test_start:test_end])
)

print(res)

rmse = np.sqrt(mean_squared_error(data.loc[test_start:test_end, 'Humidity (%)'], res))
print(f'RMSE: {rmse}')

fig, ax = plt.subplots(figsize=(7,3))
res.loc[test_start:test_end].plot(ax=ax, label='Predict')
data.loc[test_start:test_end, 'Humidity (%)'].plot(ax=ax, label='Test')
ax.legend()
plt.show()
