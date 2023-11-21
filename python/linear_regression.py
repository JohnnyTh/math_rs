import time
import numpy as np

import matplotlib.pyplot as plt
from sklearn.linear_model import LinearRegression


class LinearRegressionLS:
    # Least Squares Method
    def __init__(self):
        self.slope = None
        self.intercept = None

    def fit(self, data: np.array, labels: np.array) -> None:
        """
        Args:
            data: [N_samples, N_features]
            labels: [N_sample, N_targets]
        """
        n_elems = len(data)
        data_ = np.concatenate(
            [np.ones((n_elems, 1)), data], axis=1
        )
        xtx = np.dot(data_.T, data_)
        xy = np.dot(data_.T, labels)
        xtxi = np.linalg.inv(xtx)
        params = np.dot(xtxi, xy)

        self.slope = params[1:]
        self.intercept = params[0]

    def predict(self, data: np.array) -> np.array:
        return data * self.slope + self.intercept


def sample_data_single_feature(n_samples: int, seed=42):
    np.random.seed(seed)

    data = np.linspace(0, 100, n_samples) / 100

    modifier = np.ones(len(data)) + (
            np.random.randint(1, 5, size=len(data)) / 100
    ) * np.random.choice([-1, 1], size=len(data))

    data = data * modifier

    return data.reshape(-1, 1)


def sample_data_multi_feature(n_samples: int, n_features: int):
    data = []
    for idx in range(n_features):
        data_feature = sample_data_single_feature(n_samples, seed=idx)
        data.append(data_feature)

    return np.concatenate(data, axis=1)


def main():
    np.random.seed(42)
    n_samples = 100000
    n_features = 10

    data_train = sample_data_multi_feature(n_samples, n_features)
    modifier = np.ones(n_samples) + (
            np.random.randint(0, 10, size=n_samples) / 100
    ) * np.random.choice([-1, 1], size=n_samples)

    labels_train = data_train[:, 0] * modifier.reshape(data_train[:, 0].shape)

    estimator_ls = LinearRegressionLS()

    t_0_0 = time.perf_counter()
    estimator_ls.fit(data_train, labels_train)
    t_0_1 = time.perf_counter()

    # labels_predict = estimator_ls.predict(data_train)

    estimator = LinearRegression()

    t_1_0 = time.perf_counter()
    estimator.fit(data_train, labels_train)
    t_1_1 = time.perf_counter()

    labels_predict_sp = estimator.predict(data_train)

    # plt.scatter(data_train.flatten(), labels_train)
    # plt.plot([data_train[0], data_train[-1]], [labels_predict[0], labels_predict[-1]])
    # plt.plot([data_train[0], data_train[-1]], [labels_predict_sp[0], labels_predict_sp[-1]])
    # plt.savefig("regression_py.png")

    print(f"Data shape: {n_samples} x {n_features}")
    print(f"Least squares method time: {t_0_1 - t_0_0} s.")
    print(f"Sklearn time: {t_1_1 - t_1_0} s.")


if __name__ == "__main__":
    main()
