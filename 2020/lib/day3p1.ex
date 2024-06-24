defmodule AOC.Day3P1 do
  defp loadInput(path), do: File.read!(path)

  defp parseInput(input), do: input |> String.split("\n", trim: true)

  defp findX(nil, _), do: nil

  defp findX(row, position) do
    case position >= String.length(row) do
      true -> findX(row, rem(position, String.length(row)))
      false -> String.at(row, position)
    end
  end

  defp walkMap(rows, {right, down}) do
    Stream.unfold({0, 0}, fn {x, y} ->
      case rows |> Enum.at(y) |> findX(x) do
        "." -> {false, {x + right, y + down}}
        "#" -> {true, {x + right, y + down}}
        _ -> nil
      end
    end)
  end

  def process(path) do
    path
    |> loadInput()
    |> parseInput()
    |> walkMap({3, 1})
    |> Enum.count(& &1)
  end
end
