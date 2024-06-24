defmodule AOC.Day4P2 do
  defmodule Passport do
    defstruct [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid, :cid]
  end

  defp is_byr_valid?(nil), do: false
  defp is_byr_valid?(byr), do: String.to_integer(byr) >= 1920 and String.to_integer(byr) <= 2002

  defp is_iyr_valid?(nil), do: false
  defp is_iyr_valid?(iyr), do: String.to_integer(iyr) >= 2010 and String.to_integer(iyr) <= 2020

  defp is_eyr_valid?(nil), do: false
  defp is_eyr_valid?(eyr), do: String.to_integer(eyr) >= 2020 and String.to_integer(eyr) <= 2030

  defp is_hgt_valid?(nil), do: false
  defp is_hgt_valid?(""), do: false

  defp is_hgt_valid?(hgt) do
    if String.contains?(hgt, "cm") == false or String.contains?(hgt, "in") == false do
      false
    end

    unit = hgt |> String.slice(-2..-1)
    raw_value = hgt |> String.split(unit) |> Enum.at(0)

    value =
      try do
        String.to_integer(raw_value)
      rescue
        e ->
          IO.inspect(e)
          IO.inspect(hgt)
          false
      end

    case unit do
      "cm" ->
        value >= 150 and value <= 193

      "in" ->
        value >= 59 and value <= 76

      _ ->
        false
    end
  end

  defp is_hcl_valid?("#" <> hcl) do
    hcl
    |> String.split("", trim: true)
    |> Enum.map(&String.to_charlist/1)
    |> Enum.all?(fn c -> (c >= 'a' and c <= 'f') or (c >= '0' and c <= '9') end)
  end

  defp is_hcl_valid?(_), do: false

  defp is_ecl_valid?(ecl) do
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] |> Enum.member?(ecl)
  end

  def is_pid_valid?(nil), do: false

  def is_pid_valid?(pid) do
    String.length(pid) == 9 &&
      pid
      |> String.split("", trim: true)
      |> Enum.map(&String.to_charlist/1)
      |> Enum.all?(fn c -> c >= '0' and c <= '9' end)
  end

  defp is_cid_valid?(_cid), do: true

  @spec is_passport_valid?(%Passport{}) :: boolean()
  defp is_passport_valid?(passport) do
    required = [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid]

    if required |> Enum.all?(fn key -> Map.get(passport, key) != nil end) == false do
      false
    end

    %Passport{byr: byr, iyr: iyr, eyr: eyr, hgt: hgt, hcl: hcl, ecl: ecl, pid: pid, cid: cid} =
      passport

    v_byr = is_byr_valid?(byr)
    v_iyr = is_iyr_valid?(iyr)
    v_eyr = is_eyr_valid?(eyr)
    v_hgt = is_hgt_valid?(hgt)
    v_hcl = is_hcl_valid?(hcl)
    v_ecl = is_ecl_valid?(ecl)
    v_pid = is_pid_valid?(pid)
    v_cid = is_cid_valid?(cid)

    v_byr and v_iyr and v_eyr and v_hgt and v_hcl and v_ecl and v_pid and v_cid
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
