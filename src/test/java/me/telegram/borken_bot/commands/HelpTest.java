package me.telegram.borken_bot.commands;

import lib.TestUtils;
import me.telegram.borken_bot.GMBot;
import org.junit.Before;
import org.junit.Test;
import org.telegram.telegrambots.api.objects.Chat;
import org.telegram.telegrambots.api.objects.Message;
import org.telegram.telegrambots.api.objects.User;
import org.telegram.telegrambots.bots.AbsSender;

import java.util.Arrays;
import java.util.List;

import static org.junit.Assert.*;
import static org.mockito.Mockito.doReturn;
import static org.mockito.Mockito.mock;

public class HelpTest {
    protected Help help;
    protected GMBot bot;

    @Before
    public void setUp() throws Exception {
        bot = mock(GMBot.class);
        help = new Help(bot);
    }

    @Test
    public void testGetCommandHelp() throws Exception {
        List<AbsCommand> list = Arrays.asList(
                createCommand("test0", "do something for test0"),
                createCommand("test1", "do something for test1", new String[]{"tst1.1", "tst1.2"}),
                createCommand("test2", "do something for test2", new String[]{"tst2.2", "tst2.2"})
        );

        doReturn(list).when(bot).getRegistry();

        String expected = "/test0 - do something for test0\n\n" +
                "/test1 - do something for test1\n" +
                "/tst1.1, /tst1.2\n\n" +
                "/test2 - do something for test2\n" +
                "/tst2.2, /tst2.2";

        assertEquals(expected, help.getHelp());
    }

    protected AbsCommand createCommand(String command, String description) {
        return createCommand(command, description, null);
    }

    protected AbsCommand createCommand(String command, String description, String[] shorts) {
        return new AbsCommand(command, description) {
            @Override
            public boolean isValidAction(String action, Message message) {
                return false;
            }

            @Override
            public String[] getShortNotations() {
                return shorts;
            }

            @Override
            public void execute(AbsSender absSender, User user, Chat chat, String[] strings) {
            }
        };
    }

    @Test
    public void testIsValidAction() {
        assertTrue(help.isValidAction("help", null));
        assertFalse(help.isValidAction("notHelp", null));
    }

    @Test
    public void testConflictCommands() {
        assertFalse(TestUtils.isConflict(help));
    }
}