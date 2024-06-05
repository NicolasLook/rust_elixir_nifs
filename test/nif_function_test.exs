defmodule RustElixirNifs.NifFunctionTest do
  use ExUnit.Case
  doctest RustElixirNifs.NifFunction

  alias RustElixirNifs.NifFunction

  test "compute_all" do
    assert NifFunction.compute_all(0.5, 0.5, 0.5, 0.5, 0.5) == %{
             "call_charm" => -0.23859360492518916,
             "call_delta" => 0.8116204410942089,
             "call_gamma" => 1.52699907152121,
             "call_price" => 0.13242838339919438,
             "call_rho" => 0.13669091857395504,
             "call_theta" => -0.18440963955899287,
             "call_vanna" => -0.28631232591022676,
             "call_vega" => 0.09543744197007564,
             "call_vomma" => 0.08947260184694582,
             "put_charm" => -0.23859360492518916,
             "put_delta" => -0.18837955890579106,
             "put_gamma" => 1.52699907152121,
             "put_price" => 0.021828774934896877,
             "put_rho" => -0.05800927719389618,
             "put_theta" => 0.010290556208858367,
             "put_vanna" => -0.28631232591022676,
             "put_vega" => 0.09543744197007564,
             "put_vomma" => 0.08947260184694582
           }
  end

  test "implied_black_volatility" do
    assert NifFunction.implied_black_volatility(20.0, 100.0, 90.0, 30.0, true) ==
             0.07011701801482094
  end

  test "implied_normal_volatility" do
    assert NifFunction.implied_normal_volatility(20.0, 100.0, 90.0, 30.0, true) ==
             6.614292466299764
  end
end
