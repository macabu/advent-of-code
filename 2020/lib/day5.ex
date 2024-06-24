defmodule AOC.Day5 do
  defmodule BinarySeat do
    defstruct [:row, :col]
  end

  @spec parse_seats(binary) :: [%BinarySeat{}]
  def parse_seats(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      row = String.slice(line, 0..6)
      col = String.slice(line, -3..-1)

      %BinarySeat{row: row, col: col}
    end)
    |> Enum.map(&get_seat/1)
  end

  def reduce_seat_pos(pos, {min_init, max_init}) do
    %{min: min_pos, max: max_pos} =
      pos
      |> String.split("", trim: true)
      |> Enum.reduce(%{min: min_init, max: max_init}, fn r, %{min: min, max: max} ->
        avg = div(min + max, 2)

        case r do
          "F" -> %{min: min, max: avg}
          "B" -> %{min: avg + 1, max: max}
          "L" -> %{min: min, max: avg}
          "R" -> %{min: avg + 1, max: max}
        end
      end)

    case pos |> String.at(String.length(pos) - 1) do
      "F" -> min_pos
      "B" -> max_pos
      "L" -> min_pos
      "R" -> max_pos
    end
  end

  defp get_seat_row(rows), do: reduce_seat_pos(rows, {0, 127})
  defp get_seat_col(cols), do: reduce_seat_pos(cols, {0, 7})

  def get_seat(%BinarySeat{row: row, col: col}) do
    r = get_seat_row(row)
    c = get_seat_col(col)

    %{id: r * 8 + c, r: r, c: c}
  end

  def get_my_seat(seats) do
    sorted_seats = seats |> Enum.sort_by(fn %{id: id} -> id end, :asc)
    ids = sorted_seats |> Enum.map(fn %{id: id} -> id end)

    %{id: min_id} = sorted_seats |> Enum.at(0)
    %{id: max_id} = sorted_seats |> Enum.at(length(sorted_seats) - 1)

    min_id..max_id |> Enum.filter(fn n -> not Enum.member?(ids, n) end)
  end

  def process1(path) do
    File.read!(path)
    |> parse_seats
    |> Enum.map(fn %{id: id} -> id end)
    |> Enum.max()
  end

  def process2(path) do
    File.read!(path)
    |> parse_seats
    |> get_my_seat
  end
end
