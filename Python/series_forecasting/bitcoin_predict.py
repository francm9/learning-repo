import pandas as pd
import matplotlib.pyplot as plt
from skforecast.ForecasterAutoreg import ForecasterAutoreg
from lightgbm import LGBMRegressor
from skforecast.model_selection import bayesian_search_forecaster
import numpy as np
from sklearn.metrics import mean_squared_error

data = pd.read_csv('BTC-USD.csv')

data['Date'] = pd.to_datetime(data['Date'], format='%Y-%m-%d')
data.set_index('Date',inplace=True)
data.sort_index(inplace=True)

data = data.asfreq('D', method='bfill')
data.ffill(inplace=True)

train_start = '2020-01-06'
train_end = '2023-01-06'

test_start = '2023-01-07'
test_end = '2024-03-06'

# fig, ax = plt.subplots(figsize=(7,3))
# data.loc[train_start:train_end, ['Open']].plot(ax=ax, label='Train Open')
# data.loc[train_start:train_end, ['Close']].plot(ax=ax, label='Train Close')
# data.loc[test_start:test_end, 'Open'].plot(ax=ax, label='Test Open')
# data.loc[test_start:test_end, ['Close']].plot(ax=ax, label='Train Close')
# data.loc[train_start:train_end, ['Volume']].plot(ax=ax, label='Train Volume')
# data.loc[test_start:test_end, ['Volume']].plot(ax=ax, label='Test Volume')

# ax.legend()
# plt.show()

forecaster = ForecasterAutoreg (
    regressor=LGBMRegressor(random_state=123),
    lags=30
)

lags_grid = [10, 20, 30, 40, 50, 60 ,100, 120 ,150]

def search_space(trial):
    search_space = {
        'n_estimators': trial.suggest_int('n_estimators', 50, 200),
        'max_depth': trial.suggest_int('max_depth', -1, 15),
        'learning_rate': trial.suggest_uniform('learning_rate', 0.001, 0.15),
        'num_leaves': trial.suggest_int('num_leaves', 5, 50),
        'reg_alpha': trial.suggest_uniform('reg_alpha', 0.0, 1)
    }
    return search_space

results, frozen_trial = bayesian_search_forecaster(
    forecaster=forecaster,
    y=data.loc[train_start:test_end, 'Close'],
    search_space=search_space,
    lags_grid=lags_grid,
    steps=30,
    metric='mean_squared_error',
    initial_train_size=len(data.loc[train_start:train_end]),
    fixed_train_size=False,
    random_state=123,
    return_best=True,
    verbose=True,
    engine='optuna',
    exog=data.loc[train_start:test_end, 'Low']
)

# LGBM_Optuna = forecaster.predict(
#     steps=len(data.loc[test_start:test_end]), 
#     exog=results.loc[test_start:test_end, 'Volume']
# )

# rmse_LGBM = np.sqrt(mean_squared_error(data.loc[test_start:test_end, 'Close'], LGBM_Optuna))
# print(f'RMSE Test LGBM: {rmse_LGBM}')

