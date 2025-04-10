from aqmodels._api_utils import export, dispatched


@export
class Solution:
    @dispatched
    def __str__(self):  # type: ignore[reportIncompatibleMethodOverride]
        return

    @dispatched
    def __repr__(self):  # type: ignore[reportIncompatibleMethodOverride]
        return

    @dispatched
    def __iter__(self):
        return

    @dispatched
    def __getitem__(self, item):
        return item

    @property
    @dispatched
    def results(self):
        return

    @property
    @dispatched
    def samples(self):
        return

    @property
    @dispatched
    def obj_values(self):
        return

    @property
    @dispatched
    def raw_energies(self):
        return

    @property
    @dispatched
    def num_occurrences(self):
        return

    @property
    @dispatched
    def runtime(self):
        return

    @property
    @dispatched
    def best_sample_idx(self):
        return

    @dispatched
    def encode(self, compress=True, level=3):
        """
        Serialize the solution into a compact binary format.

        Parameters
        ----------
        compress : bool, optional
            Whether to compress the binary output. Default is True.
        level : int, optional
            Compression level (0–9). Default is 3.

        Returns
        -------
        bytes
            Encoded model representation.

        Raises
        ------
        IOError
            If serialization fails.
        """
        return compress, level

    @dispatched
    def serialize(self, compress=True, level=3):
        """Alias for `encode()`."""
        return compress, level

    @dispatched
    @staticmethod
    def decode(data):
        """
        Reconstruct a solution object from binary data.

        Parameters
        ----------
        data : bytes
            Serialized model blob created by `encode()`.

        Returns
        -------
        Solution
            The reconstructed solution.

        Raises
        ------
        DecodeError
            If decoding fails due to corruption or incompatibility.
        """
        return data

    @dispatched
    @staticmethod
    def deserialize(data):
        """Alias for `decode()`."""
        return data
