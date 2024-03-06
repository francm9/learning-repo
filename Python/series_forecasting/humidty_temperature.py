import pandas as pd
import matplotlib.pyplot as plt
from skforecast.ForecasterAutoreg import ForecasterAutoreg
from lightgbm import LGBMRegressor
from skforecast.model_selection import bayesian_search_forecaster
import numpy as np
from sklearn.metrics import mean_squared_error
from sklearn.tree import DecisionTreeRegressor

data = pd.read_csv('Crop_recommendation.csv')
data = data[data['label'] == 'rice']
data.drop(['label'], axis=1)

train_start = 0
train_end = 80
test_start = 81
test_end = 99

forecaster = ForecasterAutoreg(
    regressor = DecisionTreeRegressor(random_state=123),
    lags = 30
)

forecaster.fit(
    y=data.loc[train_start:train_end, 'humidity']
)

res = forecaster.predict(
    steps=len(data.loc[test_start:test_end, 'humidity'])
)


fig, ax = plt.subplots(figsize=(7,3))
data.loc[test_start:test_end, 'humidity'].plot(ax=ax, label='Test')
res.plot(ax=ax, label='Predict')
ax.legend()
plt.show()
