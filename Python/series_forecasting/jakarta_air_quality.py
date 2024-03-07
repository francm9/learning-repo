import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl
from skforecast.ForecasterAutoreg import ForecasterAutoreg
from sklearn.tree import DecisionTreeRegressor
import numpy as np

# Cargar el csv
data = pd.read_csv('jakarta-south (us consulate), indonesia-air-quality.csv')

#Formatear la fecha y usarla como índice
data['date'] = pd.to_datetime(data['date'], format='%Y/%m/%d')
data.set_index('date',inplace=True)
data.sort_index(inplace=True)

#Cambiar la frecuencia a diaria y rellenar los valores faltantes
data = data.asfreq('D', method='bfill')

#Cambiar el tipo a float
#Los errores se convierten en NaN
data = pd.to_numeric(data.loc['2020-01-01':'2024-03-04', ' pm25'], errors='coerce')

#Rellenar los valores faltantes con el valor anterior
data.ffill(inplace=True)

#Crear conjuntos de test, entrenamiento y predicción
train_start = '2020-01-01'
train_end = '2022-12-31'

test_start = '2023-01-01'
test_end = '2023-12-31'

predict_start = '2024-01-01'
predict_end = '2024-03-04'

#Visualizar los datos
# fig, ax = plt.subplots(figsize=(7,3))
# data.loc[train_start:train_end].plot(ax=ax, label='Train')
# data.loc[test_start:test_end].plot(ax=ax, label='Test')
# data.loc[predict_start:predict_end].plot(ax=ax, label='Predict')
# ax.legend()

# #Crear un forecaster
# forecaster = ForecasterAutoreg(
#     regressor = DecisionTreeRegressor(random_state=123),
#     lags = 30
# )

# forecaster.fit(y=data.loc[train_start:train_end])

# #Test
# res_test = forecaster.predict(steps=len(data.loc[test_start:test_end]))

# fig, ax = plt.subplots(figsize=(7,3))
# data.loc[test_start:test_end].plot(ax=ax, label='Test')
# res_test.loc[test_start:test_end].plot(ax=ax, label='Predict')
# ax.legend()

# #RMSE
# from sklearn.metrics import mean_squared_error

# rmse_train = np.sqrt(np.mean(np.square(forecaster.in_sample_residuals)))
# rmse_test = np.sqrt(mean_squared_error(data.loc[test_start:test_end], res_test))

# print(f'RMSE Train: {rmse_train} - RMSE Test: {rmse_test}')

#Backtesting - cross-validation
#Backtesting donde el conjunto de entrenamiento es más grande en cada iteración
# from skforecast.model_selection import backtesting_forecaster

# metric, predicted_backtest = backtesting_forecaster(
#     forecaster = forecaster,
#     y = data.loc[train_start:predict_end],
#     steps = 30,
#     metric = 'mean_squared_error',
#     initial_train_size = len(data.loc[train_start:train_end]),
#     fixed_train_size = False,
#     allow_incomplete_fold = True,
#     refit = True
# )

# print(np.sqrt(metric))

# #Hyperparameter tuning
# from skforecast.model_selection import grid_search_forecaster

# #ccp_alpha es el parámetro que elige los árboles de mayor complejidad menores que el valor dado
# param_grid = {
#     'max_depth': [None, 1, 3, 5],
#     'min_samples_split': [2, 3, 4],
#     'ccp_alpha': [0.0, 0.001, 0.01]
# }

# lags_grid = [30, 50, 100, 150, [1,2,3,4,5,7,9,13,15,20,50]]

# res = grid_search_forecaster(
#     forecaster = forecaster,
#     y = data.loc[train_start:predict_end],
#     param_grid = param_grid,
#     lags_grid = lags_grid,
#     steps = 30,
#     refit=True,
#     metric = 'mean_squared_error',
#     initial_train_size = len(data.loc[train_start:train_end]),
#     fixed_train_size=False,
#     return_best=True,
#     verbose=False
# )

# #Testing the best model
# forecaster2 = ForecasterAutoreg(
#     regressor = DecisionTreeRegressor(
#         max_depth=5,
#         min_samples_split=4,
#         ccp_alpha=0.01,
#     ),
#     lags = 50,
# )

# forecaster2.fit(y=data.loc[train_start:train_end])

# res_test2 = forecaster2.predict(steps=len(data.loc[test_start:test_end]))

# rmse_test2 = np.sqrt(mean_squared_error(data.loc[test_start:test_end], res_test2))

# print(f'RMSE Test 2: {rmse_test2}')

#Hyperparameter tuning: optuna/bayesian search
# from lightgbm import LGBMRegressor
# from skforecast.model_selection import bayesian_search_forecaster


# forecaster = ForecasterAutoreg(
#     regressor = LGBMRegressor(random_state=123),
#     lags = 30
# )

# # Lags Grid
# lags_grid = [30, 50, 100, 150, [1,2,3,4,5,7,9,13,15,20,50]]

# def search_space(trial):
#     search_space = {
#         'num_leaves': trial.suggest_int('num_leaves', 10, 31),                  # Número máximo de hojas
#         'max_depth': trial.suggest_int('max_depth', -1, 10),                # Máxima profundidad del árbol (0 es sin límite)
#         'learning_rate': trial.suggest_float('learning_rate', 0.001, 0.1),  # Tasa de aprendizaje (se puede adaptar en entrenamiento)
#         'n_estimators': trial.suggest_int('n_estimators', 50, 200),         # Número de árboles
#         'reg_alpha': trial.suggest_float('reg_alpha', 0.0, 0.5),            # Notion
#     }
#     return search_space

# results, frozen_trial = bayesian_search_forecaster(
#     forecaster = forecaster,
#     y = data.loc[train_start:predict_end],
#     search_space = search_space,
#     lags_grid = lags_grid,
#     steps = 30,
#     metric = 'mean_squared_error',
#     initial_train_size = len(data.loc[train_start:train_end]),
#     fixed_train_size = False,
#     return_best=True,
#     refit=True,
#     random_state=123,
#     verbose = False,
#     engine='optuna'
# )


# LGBM_Optuna = forecaster.predict(steps=len(data.loc[test_start:test_end]))
# rmse_LGBM = np.sqrt(mean_squared_error(data.loc[test_start:test_end], LGBM_Optuna))
# print(f'RMSE Test LGBM: {rmse_LGBM}, RMSE Test DecisionTree: {rmse_test}')

# Variables exogenas
exog_1 = np.log(np.float_power(data, 10))
exog_2 = np.float_power(data, 0.75) + 50 + np.random.normal(scale=10)

#Mezclar todo
data_exog = pd.concat([data, exog_1, exog_2], axis=1)
data_exog.columns = ['y', 'exog_1', 'exog_2']

print(data_exog)

# forecaster_exog = ForecasterAutoreg(
#     regressor=DecisionTreeRegressor(random_state=123),
#     lags=30
# )

# forecaster_exog.fit(
#     y = data_exog.loc[train_start:train_end, 'y'],
#     exog= data_exog.loc[train_start:train_end, ['exog_1', 'exog_2']]
# )

# res_test_exog = forecaster_exog.predict(
#     steps=len(data_exog.loc[test_start:test_end]),
#     exog=data_exog.loc[test_start:test_end, ['exog_1', 'exog_2']]
# )

# # Visualizar
# mpl.use('TkAgg')
# fig, ax = plt.subplots(figsize=(7,3))
# data_exog.loc[train_start:test_end, 'y'].plot(ax=ax, label='Test')
# res_test_exog.plot(ax=ax, label='Predict')
# ax.legend()
# plt.show()
