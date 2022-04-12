defmodule FactorialTest do
  use ExUnit.Case
  doctest Factorial

  test "greets the world" do
    assert Factorial.hello() == :world
  end
end
