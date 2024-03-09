import pandas as pd
import matplotlib.pyplot as plt
from skforecast.ForecasterAutoreg import ForecasterAutoreg
from sklearn.ensemble import RandomForestRegressor
from sklearn.tree import DecisionTreeRegressor
import numpy as np
from sklearn.metrics import mean_squared_error

# Load data
data = pd.read_csv('soil-moisture.csv')


# Crear una columna para el año
for i in range(len(data)):
    year = 2023
    if data.loc[i, 'Month'] == 'Jan' or data.loc[i, 'Month'] == 'Feb' or data.loc[i, 'Month'] == 'Mar':
        year = 2024
    else :
        year = 2023

    if (data.loc[i, 'Month'] == 'Jan'): data.loc[i, 'Month'] = 1
    elif (data.loc[i, 'Month'] == 'Feb'): data.loc[i, 'Month'] = 2
    elif (data.loc[i, 'Month'] == 'Mar'): data.loc[i, 'Month'] = 3
    elif (data.loc[i, 'Month'] == 'Apr'): data.loc[i, 'Month'] = 4
    elif (data.loc[i, 'Month'] == 'May'): data.loc[i, 'Month'] = 5
    elif (data.loc[i, 'Month'] == 'Jun'): data.loc[i, 'Month'] = 6
    elif (data.loc[i, 'Month'] == 'Jul'): data.loc[i, 'Month'] = 7
    elif (data.loc[i, 'Month'] == 'Aug'): data.loc[i, 'Month'] = 8
    elif (data.loc[i, 'Month'] == 'Sep'): data.loc[i, 'Month'] = 9
    elif (data.loc[i, 'Month'] == 'Oct'): data.loc[i, 'Month'] = 10
    elif (data.loc[i, 'Month'] == 'Nov'): data.loc[i, 'Month'] = 11
    elif (data.loc[i, 'Month'] == 'Dec'): data.loc[i, 'Month'] = 12
    data.loc[i, 'Year'] = year

data['Date'] = pd.to_datetime(data[['Year', 'Month', 'Day']])
data.drop(['Year', 'Month', 'Day'], axis=1, inplace=True)
data.set_index('Date',inplace=True)
data.sort_index(inplace=True)

data = data.asfreq('D', method='bfill')

train_start = '2023-07-18'
train_end = '2023-12-31'
test_start = '2024-01-01'
test_end = '2024-03-10'
####################### VAR ############################
########################################################
## crea una prediccion estacionaria

# from statsmodels.tsa.vector_ar.var_model import VAR

# model = VAR(data.loc[train_start:test_end])
# model_fit = model.fit(ic='bic', maxlags=7)

# prediction = model_fit.forecast(y=model.endog, steps=len(data.loc[test_start:test_end]))
# prediction_humidity = []
# prediction_temp = []

# for i in range(len(prediction)):
#     prediction_humidity.append(prediction[i][6])
#     prediction_temp.append(prediction[i][5])

# hum_df = pd.DataFrame(prediction_humidity)
# temp_df = pd.DataFrame(prediction_temp)



# real_temp_df = []
# real_hum_df = []
# for i in range(len(data.loc[test_start:test_end])):
#     real_temp_df.append(data.loc[test_start:test_end, 'avg_temp'].iloc[i])
#     real_hum_df.append(data.loc[test_start:test_end, 'avg_humd'].iloc[i])

# real_temp_df = pd.DataFrame(real_temp_df)
# real_hum_df = pd.DataFrame(real_hum_df)

# fig,ax = plt.subplots(figsize=(12,6))
# ax.plot(hum_df, label='Real Temperature')
# ax.plot(real_hum_df, label='Predicted Temperature')
# plt.show()

##################################################################################################################
######################################### ARIMA,SARIMA ####################################################################
# import warnings
# import itertools
# import pmdarima as pm
# import statsmodels.api as sm

# # Comprobar la tendencia, varianza y estacionalidad de los datos
# from pylab import rcParams

# rcParams['figure.figsize'] = 18, 8
# # model: tipo de componente estacional - additive o multiplicative
# decomposition = sm.tsa.seasonal_decompose(data['avg_humd'], model='additive')
# fig = decomposition.plot()
# plt.show()

# Eliminar la estacionalidad
# from statsmodels.tsa.stattools import adfuller

# def adf_test(timeseries):
    # # Dickey-Fuller test
    # print('Results of Dickey-Fuller Test:')
    # dftest = adfuller(timeseries, autolag='AIC')
    # dfoutput = pd.Series(dftest[0:4], index=['Test Statistic', 'p-value', '#Lags Used', 'Number of Observations Used'])
    # for key, value in dftest[4].items():
        # dfoutput['Critical Value (%s)' % key] = value
    # print(dfoutput)

# # Diferenciación de primer orden
# #ndiffs
# data_diff = data['avg_humd'] - data['avg_humd'].shift(1)
# data_diff = data_diff.dropna()

# #nsdiffs
# data_diff = data_diff - data_diff.shift(6)
# data_diff = data_diff.dropna()

# ACF and PACF plots --- SARIMA(1,1,1)x(1,1,1)6
# from statsmodels.graphics.tsaplots import plot_acf, plot_pacf
# plot_acf(data_diff)
# plt.show()
# plot_pacf(data_diff)
# plt.show()

#Model creation
# Se hace una búsqueda para ver cuales son los mejores parámetros para el modelo
# p = range(0, 3)
# d = range(1, 2)
# q = range(0, 3)

# pdq = list(itertools.product(p, d, q))
# seasonal_pdq = [(x[0], x[1], x[2], 6) for x in list(itertools.product(p, d, q))]
# print('Examples of parameter combinations for Seasonal ARIMA...')
# print('SARIMAX: {} x {}'.format(pdq[1], seasonal_pdq[1]))
# print('SARIMAX: {} x {}'.format(pdq[1], seasonal_pdq[2]))
# print('SARIMAX: {} x {}'.format(pdq[2], seasonal_pdq[3]))
# print('SARIMAX: {} x {}'.format(pdq[2], seasonal_pdq[4]))

# for param in pdq:
    # for param_seasonal in seasonal_pdq:
        # try:
            # mod = sm.tsa.statespace.SARIMAX(data['avg_humd'],
                                            # order=param,
                                            # seasonal_order=param_seasonal,
                                            # enforce_stationarity=False,
                                            # enforce_invertibility=False)

            # results = mod.fit()
            # print('ARIMA{}x{}6 - AIC:{}'.format(param, param_seasonal, results.aic))
        # except:
            # continue
########################## ForecasterMultivariate ########################
###########################################################################
from skforecast.ForecasterAutoregMultiVariate import ForecasterAutoregMultiVariate
from skforecast.model_selection_multiseries import random_search_forecaster_multiseries
from lightgbm import LGBMRegressor

data_train = data.loc[train_start:train_end].copy()

forecaster = ForecasterAutoregMultiVariate(
    regressor = RandomForestRegressor(random_state=123),
    lags=30,
    level='avg_humd',
    steps=len(data.loc[test_start:test_end]),
    n_jobs='auto'
)

lags_grid = [10, 20, 30, 40, 50]
param_distributions = {
    'n_estimators': [100, 200, 300, 400, 500],
    'max_depth': [1,2,3,4,5],
}

grid_search = random_search_forecaster_multiseries(
    forecaster=forecaster,
    series=data,
    lags_grid=lags_grid,
    param_distributions=param_distributions,
    steps=4,
    metric='mean_squared_error',
    initial_train_size=len(data_train),
    allow_incomplete_fold=True,
    return_best=True,
    refit=False,
    n_jobs='auto',
    verbose=False,
    show_progress=True
)



# forecaster.fit(series=data_train)
res = forecaster.predict(steps=len(data.loc[test_start:test_end]), last_window=data_train)

# Error
rmse = np.sqrt(mean_squared_error(data.loc[test_start:test_end, 'avg_humd'], res))
print(f'RMSE: {rmse}')

# Plot
fig, ax = plt.subplots(figsize=(12, 6))
data['avg_humd'].loc[test_start:test_end].plot(ax=ax, label='Real')
res.plot(ax=ax, label='Predicted')
plt.show()
