defmodule AOC.Day2P2 do
  defmodule Rule do
    defstruct [:p1, :p2, :c]
  end

  defmodule PasswordRule do
    defstruct rule: %Rule{}, password: ""
  end

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

      %PasswordRule{rule: rule, password: password}
    end)
  end

  defp parsePasswordRule(rule) do
    [positions, char] = rule |> String.split()

    [p1, p2] = positions |> String.split("-") |> Enum.map(&String.to_integer/1)

    case p1 <= 0 do
      true -> %Rule{p1: 0, p2: p2 - 1, c: char}
      false -> %Rule{p1: p1 - 1, p2: p2 - 1, c: char}
    end
  end

  defp validPasswordCount(passwordRules) do
    passwordRules
    |> Enum.map(fn %PasswordRule{password: password, rule: %Rule{p1: p1, p2: p2, c: requiredChar}} ->
      p1Exists? = password |> String.at(p1) == requiredChar
      p2Exists? = password |> String.at(p2) == requiredChar

      case p1Exists? do
        true ->
          case p2Exists? do
            true -> false
            false -> true
          end

        false ->
          p2Exists?
      end
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
