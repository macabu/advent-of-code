defmodule AOC.Day1 do
  defp loadInput(path) do
    {:ok, input} = File.read(path)
    input
  end

  defp takeFirst(inputList) do
    inputList
    |> Enum.take(1)
  end

  defp multiplyPairs(inputList) do
    inputList
    |> Enum.map(fn %{a: a, b: b} -> a * b end)
  end

  defp multiplyTriples(inputList) do
    inputList
    |> Enum.map(fn %{a: a, b: b, c: c} -> a * b * c end)
  end

  defp sumTo2020?(inputList) do
    inputList
    |> Enum.filter(fn %{sum: sum} -> sum == 2020 end)
  end

  defp sumFromItem(item, inputList) do
    inputList
    |> Enum.map(fn x -> %{a: x, b: item, sum: x + item} end)
  end

  defp sumThree(item, inputList) do
    inputList
    |> Enum.map(fn %{a: a, b: b, sum: sum} -> %{a: a, b: b, c: item, sum: sum + item} end)
  end

  defp sumInput(inputList) do
    byTwo = inputList |> Enum.flat_map(fn x -> sumFromItem(x, inputList) end)
    byThree = inputList |> Enum.flat_map(fn x -> sumThree(x, byTwo) end)
    byThree
  end

  defp formatInput(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn x -> String.to_integer(x) end)
  end

  def process(path) do
    path
    |> loadInput
    |> formatInput
    |> sumInput
    |> sumTo2020?
    |> multiplyTriples
    |> takeFirst
  end
end
