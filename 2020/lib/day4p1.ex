defmodule AOC.Day4P1 do
  defmodule Passport do
    defstruct [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid, :cid]
  end

  @spec is_passport_valid?(%Passport{}) :: boolean()
  defp is_passport_valid?(passport) do
    [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid]
    |> Enum.all?(fn key -> Map.get(passport, key) != nil end)
  end

  defp parse_passports(input) do
    input
    |> String.split("\n\n")
    |> Enum.map(&String.split(&1, [" ", "\n"]))
    |> Enum.map(fn passport ->
      Kernel.struct(
        Passport,
        passport
        |> Enum.map(fn field_value -> field_value |> String.split(":") end)
        |> Enum.map(fn [k, v] -> %{k => v} |> Map.new(fn {k, v} -> {String.to_atom(k), v} end) end)
        |> Enum.reduce(&Map.merge(&1, &2))
      )
    end)
  end

  def process(path) do
    File.read!(path)
    |> parse_passports
    |> Enum.map(&is_passport_valid?/1)
    |> Enum.reduce(0, fn valid, acc ->
      case valid do
        true -> acc + 1
        false -> acc
      end
    end)
  end
end
