defmodule AOC.Day6 do
  def process1(path) do
    File.read!(path)
    |> String.split("\n\n", trim: true)
    |> Enum.map(&String.replace(&1, "\n", ""))
    |> Enum.map(fn group ->
      group
      |> String.split("", trim: true)
      |> Enum.uniq()
      |> Kernel.length()
    end)
    |> Enum.sum()
  end

  def process2(path) do
    File.read!(path)
    |> String.split("\n\n", trim: true)
    |> Enum.map(&String.split(&1, "\n", trim: true))
    |> Enum.map(fn group ->
      ?a..?z
      |> Enum.map(fn c ->
        group
        |> Enum.join()
        |> String.to_charlist()
        |> Enum.count(&(&1 == c))
      end)
      |> Enum.count(&(&1 == Kernel.length(group)))
    end)
    |> Enum.sum()
  end
end
