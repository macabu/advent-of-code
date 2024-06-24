defmodule AOC.Day3P2 do
  defp loadInput(path) do
    {:ok, input} = File.read(path)
    input
  end

  defp parseInput(input) do
    input
    |> String.split("\n", trim: true)
  end

  defp findX(nil, _), do: nil

  defp findX(row, position) do
    if position >= String.length(row) do
      findX(row, rem(position, String.length(row)))
    else
      String.at(row, position)
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
    rows =
      path
      |> loadInput()
      |> parseInput()

    slope_1 = rows |> walkMap({1, 1}) |> Enum.count(& &1)
    slope_2 = rows |> walkMap({3, 1}) |> Enum.count(& &1)
    slope_3 = rows |> walkMap({5, 1}) |> Enum.count(& &1)
    slope_4 = rows |> walkMap({7, 1}) |> Enum.count(& &1)
    slope_5 = rows |> walkMap({1, 2}) |> Enum.count(& &1)

    slope_1 * slope_2 * slope_3 * slope_4 * slope_5
  end
end
