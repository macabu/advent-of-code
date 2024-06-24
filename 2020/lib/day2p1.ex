defmodule AOC.Day2P1 do
  defp loadInput(path) do
    {:ok, input} = File.read(path)
    input
  end

  defp formatInput(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn line ->
      [ruleToken, passToken] = line |> String.split(":")

      rule = ruleToken |> parsePasswordRule
      password = passToken |> String.trim_leading()

      %{rule: rule, password: password}
    end)
  end

  defp parsePasswordRule(rule) do
    [minMax, char] = rule |> String.split()

    [minC, maxC] = minMax |> String.split("-") |> Enum.map(&String.to_integer/1)

    %{min: minC, max: maxC, c: char}
  end

  defp validPasswordCount(passwordRules) do
    passwordRules
    |> Enum.map(fn %{password: password, rule: %{min: min, max: max, c: requiredChar}} ->
      charCount =
        password
        |> String.split("")
        |> Enum.filter(fn c -> c == requiredChar end)
        |> Kernel.length()

      charCount >= min and charCount <= max
    end)
    |> Enum.filter(fn p -> p end)
    |> Kernel.length()
  end

  def process(path) do
    path
    |> loadInput
    |> formatInput
    |> validPasswordCount
  end
end
