defmodule Factorial do
  def fact(0), do: 1
  def fact(1), do: 1

  def fact(n) do
    n * fact(n - 1)
  end

  def binomial(n, k) do
    floor(fact(n) / (fact(k) * fact(n - k)))
  end

  def fact_set(n) do
    n..1
    |> Enum.flat_map(&Factorial.PrimeExpander.expand/1) 
  end

  def binomial_via_set(n, k) do
    ((Factorial.fact_set(n) -- Factorial.fact_set(k)) -- Factorial.fact_set(n-k)) 
    |> IO.inspect()
    |> Enum.product()
  end
end

defmodule Factorial.PrimeExpander do
  def expand(0), do: []
  def expand(1), do: [1]
  def expand(2), do: [2]
  def expand(3), do: [3]
  def expand(4), do: [2, 2]
  def expand(5), do: [5]
  def expand(6), do: [2, 3]
  def expand(7), do: [7]
  def expand(8), do: [2, 2, 2]
  def expand(9), do: [3, 3]
  def expand(10), do: [2, 5]

  def expand(n) do
    2..(floor(:math.sqrt(n)) + 1)
    |> Enum.reduce_while(n, fn x, n ->
      if rem(n, x) == 0 do
        n = floor(n / x)
        {:halt, [x | expand(n)]}
      else
        {:cont, n}
      end
    end)
    |> List.wrap()
  end
end
