defmodule RustElixirNifs.NifFunction do
  @moduledoc """
  Documentation for `RustElixirNifs`.
  """

  use Rustler, otp_app: :rust_elixir_nifs, crate: "niffunction"

  def compute_all(_stock, _strike, _rate, _sigma, _maturity),
    do: :erlang.nif_error(:nif_not_loaded)

  def implied_black_volatility(_option_price, _forward, _strike, _expiry, _is_call),
    do: :erlang.nif_error(:nif_not_loaded)

  def implied_normal_volatility(_option_price, _forward, _strike, _expiry, _is_call),
    do: :erlang.nif_error(:nif_not_loaded)
end
