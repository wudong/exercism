package tournament

import (
	"bufio"
	"fmt"
	"io"
	"sort"
	"strings"
)

type Team struct {
	name string
	play int
	win  int
	lost int
	draw int
	score int
}

func Tally(in io.Reader, out io.Writer) error {
	reader := bufio.NewReader(in)

	var stat map[string]*Team = map[string]*Team{}

	for {
		line, err := reader.ReadString('\n')

		fmt.Printf("reading line: %s", line)
		if err != nil && err != io.EOF {
			break
		}

		if len(strings.TrimSpace(line))!=0 {
			processLine(line, &stat)
		}

		if err == io.EOF {
			break
		}
	}

	teams:= make([]*Team, 0, len(stat))
	for _, value:=range stat {
		teams = append(teams, value)
	}

	sort.Slice(teams, func(i, j int) bool {
		if teams[i].score > teams[j].score{
			return true
		}else if teams[i].score == teams[j].score {
			return teams[i].name < teams[j].name
		}else {
			return false
		}
	})

	writer := bufio.NewWriter(out)
	writer.WriteString(fmt.Sprintf("%-31s|%3s |%3s |%3s |%3s |%3s\n", "Team", "MP", "W", "D", "L", "P"))

	for _, team:=range teams {
		writer.WriteString(fmt.Sprintf("%-31s| %2d | %2d | %2d | %2d | %2d\n", team.name, team.play, team.win, team.draw, team.lost, team.score))
	}

	writer.Flush()

	return nil
}

func processLine(line string, m *map[string]*Team) {
	if strings.HasPrefix(line,"#"){
		return
	}

	splits := strings.Split(line, ";")
	result := strings.TrimSpace(splits[2])
	switch result {
	case "win":
		{
			check(m, splits[0], 1)
			check(m, splits[1], 3)
		}
	case "loss":
		{
			check(m, splits[0], 3)
			check(m, splits[1], 1)
		}
	case "draw":
		{
			check(m, splits[0], 2)
			check(m, splits[1], 2)
		}
	default:
		fmt.Printf("line is not processed.\n")
	}

}

func check(m *map[string]*Team, name string, winrawlost byte) {
	//fmt.Printf("checking team: %s : %d\n", name, winrawlost)
	team, err := (*m)[name]
	if !err { //does not exist
		team = &Team{
			name: name,
			score: 0,
			play: 0,
			lost: 0,
			win: 0,
			draw: 0,
		}
		(*m)[team.name] = team
	}
	team.play += 1
	switch winrawlost {
	case 1:
		team.win += 1
		team.score += 3
	case 2:
		team.draw += 1
		team.score += 1
	case 3:
		team.lost += 1
	default:
	}
}
